mod entity_impls;
mod nine_patch;

use std::{borrow::Cow, collections::HashMap, f32::consts::PI, sync::OnceLock};

use super::SpriteDesc;
use anyhow::{bail, ensure, Context, Result};
use celesteloader::map::{Entity, Room};
use tiny_skia::{BlendMode, Color, Paint, PathBuilder, Rect, Stroke, Transform};

use crate::{
    asset::{AssetDb, LookupAsset},
    rendering::tileset::AIR,
    CelesteRenderData,
};

use self::nine_patch::{nine_patch, NinePatchOptions};

use super::{tileset::Matrix, RenderContext};

fn _coordinate_seed(x: f32, y: f32) -> u32 {
    // TODO make sure this is correct
    let shl = |x: f32, y: f32| x.to_bits() << y.to_bits();

    shl(x, f32::ceil(f32::log2(y.abs() + 1.0))) + y.abs() as u32
}

fn to_tile(val: f32) -> i32 {
    (val / 8.0).floor() as i32 + 1
}

pub(super) fn render_entity<L: LookupAsset>(
    r: &mut RenderContext<L>,
    fgtiles: &Matrix<char>,
    cx: &CelesteRenderData,
    asset_db: &mut AssetDb<L>,
    room: &Room,
    entity: &Entity,
) -> Result<bool> {
    let map_pos = room.bounds.position.offset_f32(entity.position);

    let entity_impls = texture_map();
    if let Some(method) = entity_impls.get(entity.name.as_str()) {
        match method {
            RenderMethod::Texture {
                texture,
                justification,
                rotation,
            } => {
                let sprite = match asset_db
                    .lookup_gameplay(cx, texture)
                    .context(entity.name.clone())
                {
                    Ok(sprite) => sprite,
                    Err(_) => return Ok(false),
                };
                r.sprite(
                    cx,
                    map_pos,
                    sprite,
                    SpriteDesc {
                        justify: justification.unwrap_or((0.5, 0.5)),
                        rotation: rotation.unwrap_or(0.0),
                        ..Default::default()
                    },
                )?;
            }
            RenderMethod::Rect { fill, border } => {
                if let Err(e) =
                    simple_outline(entity, r, map_pos, *fill, *border, BlendMode::default())
                {
                    eprintln!("failed to render rect: {e:?}");
                }
            }
            RenderMethod::FakeTiles {
                material_key,
                blend_key,
                layer,
                color: _, // TODO tint
                x,
                y,
            } => {
                ensure!(x.is_none(), "faketile x not supported yet");
                ensure!(y.is_none(), "faketile y not supported yet");
                // ensure!(color.is_none(), "faketile color not supported yet");

                let tilesets = match layer.unwrap_or("tilesFg") {
                    "tilesFg" => &cx.map_tileset.tileset_fg,
                    other => todo!("{}", other),
                };

                let target_pos = entity.position;
                let _tile_pos = (to_tile(target_pos.0), to_tile(target_pos.1));

                let width = entity.raw.get_attr_int("width")? as u32;
                let height = entity.raw.get_attr_int("height")? as u32;
                let (width_tiles, height_tiles) = (width.div_ceil(8), height.div_ceil(8));

                let material = if material_key.chars().count() == 1 {
                    material_key.chars().next().unwrap()
                } else {
                    entity
                        .raw
                        .try_get_attr_char(&material_key)?
                        .unwrap_or_else(|| {
                            eprintln!(
                                "{:?} {} has {} without {}",
                                r.area_id, room.name, entity.name, material_key
                            );
                            '3'
                        })
                };

                let draw_extra_around = 2;

                let tiles = Matrix::from_fn(
                    width_tiles + draw_extra_around * 2,
                    height_tiles + draw_extra_around * 2,
                    |x, y| {
                        if x < draw_extra_around || x >= width_tiles + draw_extra_around {
                            return '0';
                        }
                        if y < draw_extra_around || y >= height_tiles + draw_extra_around {
                            return '0';
                        }
                        // todo blend in

                        material
                    },
                );

                let _blend_mode = blend_key;

                let pos = room
                    .bounds
                    .position
                    .offset(target_pos.0 as i32, target_pos.1 as i32);
                r.render_tileset_inner(
                    (
                        width_tiles + draw_extra_around * 2,
                        height_tiles + draw_extra_around * 2,
                    ),
                    pos.offset_tile(-(draw_extra_around as i32), -(draw_extra_around as i32)),
                    &tiles,
                    tilesets,
                    cx,
                    asset_db,
                )?;
            }
        }
        return Ok(true);
    }

    match entity.name.as_str() {
        "hahaha" | "player" | "coreMessage" => return Ok(true),
        "flutterbird" => {
            let colors = ["89FBFF", "F0FC6C", "F493FF", "93BAFF"];
            let color = parse_color(fastrand::choice(colors).unwrap())?;

            let asset = asset_db.lookup_gameplay(cx, "scenery/flutterbird/idle00")?;
            r.sprite(
                cx,
                map_pos,
                asset,
                SpriteDesc {
                    justify: (0.5, 1.0),
                    tint: Some(color),
                    ..Default::default()
                },
            )?;
        }
        "jumpThru" => jump_thru(entity, fgtiles, asset_db, cx, r, map_pos)?,
        "lamp" => {
            let broken = entity.raw.get_attr::<bool>("broken")?;
            let sprite = asset_db.lookup_gameplay(cx, "scenery/lamp")?;

            let width = sprite.width() / 2;
            let half_width = width / 2;
            let height = sprite.height();

            let quad_x = if broken { width } else { 0 };

            r.sprite(
                cx,
                (map_pos.0 - half_width as f32, map_pos.1 - height as f32),
                sprite,
                SpriteDesc {
                    justify: (0.0, 0.0),
                    quad: Some((quad_x, 0, width, height)),
                    ..Default::default()
                },
            )?;
        }
        "npc" => {
            let npc = entity
                .raw
                .try_get_attr::<&str>("npc")?
                .unwrap_or("granny_00_house");

            let character = npc
                .split("_")
                .next()
                .expect("npc name has no '_'")
                .to_lowercase();

            let texture = match character.as_str() {
                "granny" => "characters/oldlady/idle00",
                "theo" => "characters/theo/theo00",
                "oshiro" => "characters/oshiro/oshiro24",
                "evil" => "characters/badeline/sleep00",
                "badeline" => "characters/badeline/sleep00",
                "gravestone" => "characters/oldlady/idle00", // TODO
                _ => bail!("unknown vanilla npc: {:?}", npc),
            };

            let sprite = asset_db.lookup_gameplay(cx, texture)?;
            r.sprite(
                cx,
                map_pos,
                sprite,
                SpriteDesc {
                    justify: (0.5, 1.0),
                    ..Default::default()
                },
            )?;
        }
        "wire" => wire(room, entity, r)?,
        "refill" => {
            let two_dash = entity.raw.try_get_attr::<bool>("twoDash")?.unwrap_or(false);
            let texture = match two_dash {
                true => "objects/refillTwo/idle00",
                false => "objects/refill/idle00",
            };

            let sprite = asset_db.lookup_gameplay(cx, texture)?;
            r.sprite(cx, map_pos, sprite, SpriteDesc::default())?;
        }
        "bonfire" => {
            let mode = entity.raw.try_get_attr::<&str>("mode")?.unwrap_or("lit");

            let texture = match mode {
                "lit" => "objects/campfire/fire08",
                "smoking" => "objects/campfire/smoking04",
                _ => "objects/campfire/fire00",
            };

            let sprite = asset_db.lookup_gameplay(cx, texture)?;
            r.sprite(
                cx,
                map_pos,
                sprite,
                SpriteDesc {
                    justify: (0.5, 1.0),
                    ..Default::default()
                },
            )?;
        }
        "strawberry" => {
            let moon = entity.raw.try_get_attr::<bool>("moon")?.unwrap_or(false);
            let winged = entity.raw.try_get_attr::<bool>("moon")?.unwrap_or(false);
            let has_nodes = !entity.nodes.is_empty();

            let texture = match (moon, winged, has_nodes) {
                (true, true, _) | (true, _, true) => "collectables/moonBerry/ghost00",
                (true, _, _) => "collectables/moonBerry/normal00",
                (false, true, true) => "collectables/ghostberry/wings01",
                (false, true, false) => "collectables/strawberry/wings01",
                (false, false, true) => "collectables/ghostberry/idle00",
                (false, false, false) => "collectables/strawberry/normal00",
            };
            let sprite = asset_db.lookup_gameplay(cx, texture)?;
            r.sprite(cx, map_pos, sprite, SpriteDesc::default())?;

            for node in &entity.nodes {
                let pos = room
                    .bounds
                    .position
                    .offset_f32((node.position.0, node.position.1));
                let sprite = asset_db.lookup_gameplay(cx, "collectables/strawberry/seed00")?;
                r.sprite(cx, pos, sprite, SpriteDesc::default())?;
            }
        }
        "goldenBerry" => {
            let winged = entity.raw.try_get_attr::<bool>("moon")?.unwrap_or(false);
            let has_nodes = !entity.nodes.is_empty();

            let texture = match (winged, has_nodes) {
                (true, true) => "collectables/ghostgoldberry/wings01",
                (true, false) => "collectables/goldberry/wings01",
                (false, true) => "collectables/ghostgoldberry/idle00",
                (false, false) => "collectables/goldberry/idle00",
            };
            let sprite = asset_db.lookup_gameplay(cx, texture)?;
            r.sprite(cx, map_pos, sprite, SpriteDesc::default())?;

            for node in &entity.nodes {
                let pos = room
                    .bounds
                    .position
                    .offset_f32((node.position.0, node.position.1));
                let sprite = asset_db.lookup_gameplay(cx, "collectables/goldberry/seed00")?;
                r.sprite(cx, pos, sprite, SpriteDesc::default())?;
            }
        }
        "blackGem" => {
            let sprite = asset_db.lookup_gameplay(cx, "collectables/heartGem/0/00")?;
            r.sprite(cx, map_pos, sprite, SpriteDesc::default())?;
        }
        "cassette" => {
            let sprite = asset_db.lookup_gameplay(cx, "collectables/cassette/idle00")?;
            r.sprite(cx, map_pos, sprite, SpriteDesc::default())?;
        }
        "checkpoint" => {
            let bg = entity
                .raw
                .attributes
                .get("bg")
                .map(|bg| bg.to_string())
                .filter(|bg| !bg.is_empty());

            let texture = bg
                .map(|bg| Cow::Owned(format!("objects/checkpoint/bg/{bg}")))
                .unwrap_or("objects/checkpoint/flash03".into());

            let sprite = asset_db.lookup_gameplay(cx, &texture)?;
            r.sprite(
                cx,
                map_pos,
                sprite,
                SpriteDesc {
                    justify: (0.5, 1.0),
                    ..Default::default()
                },
            )?;
        }
        "birdForsakenCityGem" => {
            let dish = asset_db.lookup_gameplay(cx, "objects/citysatellite/dish")?;
            r.sprite(
                cx,
                map_pos,
                dish,
                SpriteDesc {
                    justify: (0.5, 1.0),
                    ..Default::default()
                },
            )?;

            let light = asset_db.lookup_gameplay(cx, "objects/citysatellite/light")?;
            r.sprite(
                cx,
                map_pos,
                light,
                SpriteDesc {
                    justify: (0.5, 1.0),
                    ..Default::default()
                },
            )?;

            let computer_offset = (32.0, 24.0);
            let computer = asset_db.lookup_gameplay(cx, "objects/citysatellite/computer")?;
            r.sprite(
                cx,
                (map_pos.0 + computer_offset.0, map_pos.1 + computer_offset.1),
                computer,
                SpriteDesc::default(),
            )?;
            let screen = asset_db.lookup_gameplay(cx, "objects/citysatellite/computerscreen")?;
            r.sprite(
                cx,
                (map_pos.0 + computer_offset.0, map_pos.1 + computer_offset.1),
                screen,
                SpriteDesc::default(),
            )?;

            let mut nodes = entity.nodes.iter();
            let birds = nodes.next().context("satellite birds")?;
            let heart = nodes.next().context("satellite heart")?;
            let bird_pos = room.bounds.position.offset_f32(birds.position);
            let heart_pos = room.bounds.position.offset_f32(heart.position);

            let heart = asset_db.lookup_gameplay(cx, "collectables/heartGem/0/00")?;
            r.sprite(cx, heart_pos, heart, SpriteDesc::default())?;

            let bird_distance = 64i32;

            for ((dir_x, dir_y), color) in [
                ((0, -1), Color::from_rgba8(240, 240, 240, 255)),
                ((1, 1), Color::from_rgba8(10, 68, 244, 255)),
                ((1, -1), Color::from_rgba8(179, 34, 0, 255)),
                ((-1, 0), Color::from_rgba8(145, 113, 242, 255)),
                ((-1, -1), Color::from_rgba8(255, 255, 55, 255)),
            ] {
                let offset_x = dir_x * bird_distance;
                let offset_y = dir_y * bird_distance;
                let magnitude = f32::sqrt((offset_x.pow(2) + offset_y.pow(2)) as f32);

                let light = asset_db.lookup_gameplay(cx, "scenery/flutterbird/flap01")?;
                r.sprite(
                    cx,
                    (
                        bird_pos.0 + (offset_x as f32 / magnitude * bird_distance as f32),
                        bird_pos.1 + (offset_y as f32 / magnitude * bird_distance as f32),
                    ),
                    light,
                    SpriteDesc {
                        tint: Some(color),
                        ..Default::default()
                    },
                )?;
            }
        }
        "memorial" => {
            let sprite = asset_db.lookup_gameplay(cx, "scenery/memorial/memorial")?;
            r.sprite(
                cx,
                map_pos,
                sprite,
                SpriteDesc {
                    justify: (0.5, 1.0),
                    ..Default::default()
                },
            )?;
        }
        "everest/memorial" => {
            let texture = entity
                .raw
                .try_get_attr::<&str>("sprite")?
                .unwrap_or("scenery/memorial/memorial");
            let sprite = asset_db.lookup_gameplay(cx, texture)?;
            r.sprite(
                cx,
                map_pos,
                sprite,
                SpriteDesc {
                    justify: (0.5, 1.0),
                    ..Default::default()
                },
            )?;
        }
        "memorialTextController" => {
            let sprite = asset_db.lookup_gameplay(cx, "collectables/goldberry/wings01")?;
            r.sprite(
                cx,
                map_pos,
                sprite,
                SpriteDesc {
                    justify: (0.5, 1.0),
                    ..Default::default()
                },
            )?;
        }
        "badelineBoost" => {
            let sprite = asset_db.lookup_gameplay(cx, "objects/badelineboost/idle00")?;
            r.sprite(
                cx,
                map_pos,
                sprite,
                SpriteDesc {
                    justify: (0.5, 1.0),
                    ..Default::default()
                },
            )?;
        }
        "booster" => {
            let red = entity.raw.try_get_attr("red")?.unwrap_or(false);

            if red {
                let sprite = asset_db.lookup_gameplay(cx, "objects/booster/boosterRed00")?;
                r.sprite(cx, map_pos, sprite, SpriteDesc::default())?;
            } else {
                let sprite = asset_db.lookup_gameplay(cx, "objects/booster/booster00")?;
                r.sprite(cx, map_pos, sprite, SpriteDesc::default())?;
            }
        }
        "cliffside_flag" => {
            let index = entity.raw.try_get_attr_int("index")?.unwrap_or(0);

            let sprite =
                asset_db.lookup_gameplay(cx, &format!("scenery/cliffside/flag{:02}", index))?;
            r.sprite(
                cx,
                map_pos,
                sprite,
                SpriteDesc {
                    justify: (0.5, 1.0),
                    ..Default::default()
                },
            )?;
        }
        "torch" => {
            let fragile = entity.raw.get_attr("startLit")?;
            let texture = match fragile {
                true => "objects/temple/litTorch03",
                false => "objects/temple/torch00",
            };

            let sprite = asset_db.lookup_gameplay(cx, texture)?;
            r.sprite(cx, map_pos, sprite, SpriteDesc::default())?;
        }
        "cloud" => {
            let fragile = entity.raw.get_attr("fragile")?;
            let texture = match fragile {
                true => "objects/clouds/fragile00",
                false => "objects/clouds/cloud00",
            };

            let sprite = asset_db.lookup_gameplay(cx, texture)?;
            r.sprite(
                cx,
                map_pos,
                sprite,
                SpriteDesc {
                    justify: (0.5, 1.0),
                    ..Default::default()
                },
            )?;
        }
        "ridgeGate" => {
            let texture = entity
                .raw
                .try_get_attr::<&str>("texture")?
                .unwrap_or("objects/ridgeGate");
            let sprite = asset_db.lookup_gameplay(cx, texture)?;
            r.sprite(
                cx,
                map_pos,
                sprite,
                SpriteDesc {
                    justify: (0.0, 0.0),
                    ..Default::default()
                },
            )?;
        }
        "bigSpinner" => {
            let sprite = asset_db.lookup_gameplay(cx, "objects/Bumper/Idle22")?;
            r.sprite(cx, map_pos, sprite, SpriteDesc::default())?;
        }
        "whiteblock" => {
            let sprite = asset_db.lookup_gameplay(cx, "objects/whiteblock")?;
            r.sprite(
                cx,
                map_pos,
                sprite,
                SpriteDesc {
                    justify: (0.0, 0.0),
                    ..Default::default()
                },
            )?;
        }
        "spikesUp" => spikes(map_pos, entity, CardinalDir::Up, false, asset_db, cx, r)?,
        "spikesDown" => spikes(map_pos, entity, CardinalDir::Down, false, asset_db, cx, r)?,
        "spikesLeft" => spikes(map_pos, entity, CardinalDir::Left, false, asset_db, cx, r)?,
        "spikesRight" => spikes(map_pos, entity, CardinalDir::Right, false, asset_db, cx, r)?,
        "triggerSpikesDown" => spikes(map_pos, entity, CardinalDir::Up, true, asset_db, cx, r)?,
        "triggerSpikesUp" => spikes(map_pos, entity, CardinalDir::Down, true, asset_db, cx, r)?,
        "triggerSpikesLeft" => spikes(map_pos, entity, CardinalDir::Left, true, asset_db, cx, r)?,
        "triggerSpikesRight" => spikes(map_pos, entity, CardinalDir::Right, true, asset_db, cx, r)?,
        "darkChaser" => {
            let sprite = asset_db.lookup_gameplay(cx, "characters/badeline/sleep00")?;
            r.sprite(
                cx,
                map_pos,
                sprite,
                SpriteDesc {
                    justify: (0.5, 1.0),
                    ..Default::default()
                },
            )?;
        }
        "wallBooster" => {
            let left = entity.raw.try_get_attr("left")?.unwrap_or(false);
            let height = entity.raw.try_get_attr_int("height")?.unwrap_or(8);
            let tile_height = height / 8;
            let (offset_x, scale_x) = match left {
                true => (0, 1),
                false => (8, -1),
            };
            let not_core_mode = entity
                .raw
                .try_get_attr::<bool>("notCoreMode")?
                .unwrap_or(false);
            let (top, middle, bottom) = match not_core_mode {
                true => (
                    "objects/wallBooster/iceTop00",
                    "objects/wallBooster/iceMid00",
                    "objects/wallBooster/iceBottom00",
                ),
                false => (
                    "objects/wallBooster/fireTop00",
                    "objects/wallBooster/fireMid00",
                    "objects/wallBooster/fireBottom00",
                ),
            };

            let middle = asset_db.lookup_gameplay(cx, middle)?;
            for i in 2..=tile_height - 1 {
                r.sprite(
                    cx,
                    (
                        map_pos.0 + offset_x as f32,
                        map_pos.1 + ((i - 1) * 8) as f32,
                    ),
                    middle,
                    SpriteDesc {
                        scale: (scale_x as f32, 1.0),
                        justify: (0.0, 0.0),
                        ..Default::default()
                    },
                )?;
            }

            let top = asset_db.lookup_gameplay(cx, top)?;
            r.sprite(
                cx,
                (map_pos.0 + offset_x as f32, map_pos.1),
                top,
                SpriteDesc {
                    scale: (scale_x as f32, 1.0),
                    justify: (0.0, 0.0),
                    ..Default::default()
                },
            )?;
            let bottom = asset_db.lookup_gameplay(cx, bottom)?;
            r.sprite(
                cx,
                (
                    map_pos.0 + offset_x as f32,
                    map_pos.1 + ((tile_height - 1) * 8) as f32,
                ),
                bottom,
                SpriteDesc {
                    scale: (scale_x as f32, 1.0),
                    justify: (0.0, 0.0),
                    ..Default::default()
                },
            )?;
        }
        "payphone" => {
            let sprite = asset_db.lookup_gameplay(cx, "scenery/payphone")?;
            r.sprite(
                cx,
                map_pos,
                sprite,
                SpriteDesc {
                    justify: (0.5, 1.0),
                    ..Default::default()
                },
            )?;
        }
        "towerviewer" => {
            let sprite = asset_db.lookup_gameplay(cx, "objects/lookout/lookout05")?;
            r.sprite(
                cx,
                map_pos,
                sprite,
                SpriteDesc {
                    justify: (0.5, 1.0),
                    ..Default::default()
                },
            )?;
        }
        "key" => {
            let sprite = asset_db.lookup_gameplay(cx, "collectables/key/idle00")?;
            r.sprite(
                cx,
                map_pos,
                sprite,
                SpriteDesc {
                    justify: (0.5, 1.0),
                    ..Default::default()
                },
            )?;
        }
        "infiniteStar" => {
            let shielded = entity.raw.try_get_attr("shielded")?.unwrap_or(false);

            let sprite = asset_db.lookup_gameplay(cx, "objects/flyFeather/idle00")?;
            r.sprite(cx, map_pos, sprite, SpriteDesc::default())?;

            if shielded {
                r.circle(map_pos, 12.0, Color::WHITE);
            }
        }
        "touchSwitch" => {
            let container = asset_db.lookup_gameplay(cx, "objects/touchswitch/container")?;
            r.sprite(cx, map_pos, container, SpriteDesc::default())?;
            let icon = asset_db.lookup_gameplay(cx, "objects/touchswitch/icon00")?;
            r.sprite(cx, map_pos, icon, SpriteDesc::default())?;
        }
        "invisibleBarrier" => {}
        "dreammirror" => {
            let frame = asset_db.lookup_gameplay(cx, "objects/mirror/frame")?;
            r.sprite(
                cx,
                map_pos,
                frame,
                SpriteDesc {
                    justify: (0.5, 1.0),
                    ..Default::default()
                },
            )?;
            let glass = asset_db.lookup_gameplay(cx, "objects/mirror/glassbreak00")?;
            r.sprite(
                cx,
                map_pos,
                glass,
                SpriteDesc {
                    justify: (0.5, 1.0),
                    ..Default::default()
                },
            )?;
        }
        "floatingDebris" => {
            let sprite = asset_db.lookup_gameplay(cx, "scenery/debris")?;
            let offset_x = fastrand::u8(0..7) * 8;

            r.sprite(
                cx,
                (map_pos.0 - 4.0, map_pos.1 - 4.0),
                sprite,
                SpriteDesc {
                    justify: (0.0, 0.0),
                    quad: Some((offset_x as i16, 0, 8, 8)),
                    ..Default::default()
                },
            )?;
        }
        "foregroundDebris" => {
            let rock_textures: &[&[_]] = &[
                &[
                    "scenery/fgdebris/rock_a00",
                    "scenery/fgdebris/rock_a01",
                    "scenery/fgdebris/rock_a02",
                ],
                &["scenery/fgdebris/rock_b00", "scenery/fgdebris/rock_b01"],
            ];
            let rock = *fastrand::choice(rock_textures).unwrap();

            for texture in rock {
                let sprite = asset_db.lookup_gameplay(cx, texture)?;
                r.sprite(cx, map_pos, sprite, SpriteDesc::default())?;
            }
        }
        "clutterCabinet" => {
            let sprite = asset_db.lookup_gameplay(cx, "objects/resortclutter/cabinet00")?;
            r.sprite(
                cx,
                (map_pos.0 + 8.0, map_pos.1 + 8.0),
                sprite,
                SpriteDesc::default(),
            )?;
        }
        "colorSwitch" => {
            let variant = entity.raw.try_get_attr::<&str>("variant")?.unwrap_or("red");

            let button = asset_db.lookup_gameplay(cx, "objects/resortclutter/clutter_button00")?;
            r.sprite(
                cx,
                (map_pos.0 + 16.0, map_pos.1 + 16.0),
                button,
                SpriteDesc {
                    justify: (0.5, 1.0),
                    ..Default::default()
                },
            )?;

            let clutter = asset_db.lookup_gameplay(
                cx,
                &format!("objects/resortclutter/icon_{}", variant.to_lowercase()),
            )?;
            r.sprite(
                cx,
                (map_pos.0 + 16.0, map_pos.1 + 8.0),
                clutter,
                SpriteDesc::default(),
            )?;
        }
        "lockBlock" => {
            let sprite_name = entity.raw.try_get_attr("sprite")?.unwrap_or("wood");
            let texture = match sprite_name {
                "temple_a" => "objects/door/lockdoorTempleA00",
                "temple_b" => "objects/door/lockdoorTempleB00",
                "moon" => "objects/door/moonDoor11",
                "wood" | _ => "objects/door/lockdoor00",
            };
            let sprite = asset_db.lookup_gameplay(cx, texture)?;
            r.sprite(
                cx,
                (map_pos.0 + 16.0, map_pos.1 + 16.0),
                sprite,
                SpriteDesc::default(),
            )?;
        }
        "friendlyghost" => {
            let sprite = asset_db.lookup_gameplay(cx, "characteres/oshiro/boss13")?;
            r.sprite(cx, map_pos, sprite, SpriteDesc::default())?;
        }
        "killbox" | "JungleHelper/FallingKillbox" | "SorbetHelper/FlagToggledKillbox" => {
            let fill = Color::from_rgba8(204, 102, 102, 204);
            let border = Color::from_rgba8(204, 102, 102, 204);

            let blend_mode = BlendMode::default();
            let width = entity.raw.try_get_attr_int("width")?.unwrap_or(8);
            let height = 32;
            let (x, y) = r.transform_pos_f32(map_pos);
            let rect = Rect::from_xywh(x, y, width as f32, height as f32).unwrap();
            r.rect(rect, fill, blend_mode);
            r.stroke_rect(rect, border);
        }
        "water" => {
            let light_blue = Color::from_rgba8(173, 216, 230, 255);
            let fill = Color::from_rgba(
                light_blue.red() * 0.3,
                light_blue.green() * 0.3,
                light_blue.blue() * 0.3,
                0.6,
            )
            .unwrap();
            let border = Color::from_rgba(
                light_blue.red() * 0.8,
                light_blue.green() * 0.8,
                light_blue.blue() * 0.8,
                0.8,
            )
            .unwrap();

            simple_outline(entity, r, map_pos, fill, border, BlendMode::Plus)?;
        }
        "spinner" => {
            spinner_main(entity, room, asset_db, cx, r, map_pos)?;
        }
        "trackSpinner" => {
            let dust_override =
                r.area_id == Some(3) || (r.area_id == Some(7) && room.name.starts_with("lvl_d-"));
            let star_override = r.area_id == Some(10);

            let dust = entity
                .raw
                .try_get_attr::<bool>("dust")?
                .unwrap_or(dust_override);
            let star = entity
                .raw
                .try_get_attr::<bool>("star")?
                .unwrap_or(star_override);

            if star {
                let sprite = asset_db.lookup_gameplay(cx, "danger/starfish13")?;
                r.sprite(cx, map_pos, sprite, SpriteDesc::default())?;
            } else if dust {
                let base = asset_db.lookup_gameplay(cx, "danger/dustcreature/base00")?;
                r.sprite(cx, map_pos, base, SpriteDesc::default())?;
                // let outline =
                // asset_db.lookup_gameplay(cx, "@Internal@/dust_creature_outlines/base00")?;
                // r.sprite_new(cx,map_pos, outline, SpriteDesc {  justify: (0.5, 0.5),   ..Default::default() })?;
            } else {
                let sprite = asset_db.lookup_gameplay(cx, "danger/blade00")?;
                r.sprite(cx, map_pos, sprite, SpriteDesc::default())?;
            }
        }
        // TODO rotateSpinner
        "fireBall" => {
            let not_core_mode = entity
                .raw
                .try_get_attr::<bool>("notCoreMode")?
                .unwrap_or(false);
            let texture = if not_core_mode {
                "objects/fireball/fireball09"
            } else {
                "objects/fireball/fireball01"
            };
            let sprite = asset_db.lookup_gameplay(cx, texture)?;
            r.sprite(cx, map_pos, sprite, SpriteDesc::default())?;
        }
        "crumbleBlock" => {
            let variant = entity.raw.try_get_attr("variant")?.unwrap_or("default");
            let texture = format!("objects/crumbleBlock/{variant}");

            let width = entity.raw.try_get_attr_int("width")?.unwrap_or(0).max(8);

            nine_patch(
                asset_db,
                cx,
                r,
                &texture,
                map_pos,
                (width as i16, 8),
                NinePatchOptions::default(),
            )?;
        }
        "bounceBlock" => {
            let not_core_mode = entity.raw.try_get_attr("notCoreMode")?.unwrap_or(false);
            let (texture_block, texture_crystal) = match not_core_mode {
                true => (
                    "objects/BumpBlockNew/ice00",
                    "objects/BumpBlockNew/ice_center00",
                ),
                false => (
                    "objects/BumpBlockNew/fire00",
                    "objects/BumpBlockNew/fire_center00",
                ),
            };

            ninepatch_middle(
                entity,
                (24, 24),
                texture_block,
                texture_crystal,
                asset_db,
                cx,
                r,
                map_pos,
                NinePatchOptions::default(),
            )?;
        }
        "switchGate" => {
            let sprite = entity.raw.try_get_attr("sprite")?.unwrap_or("block");
            let frame = format!("objects/switchgate/{sprite}");
            let middle = "objects/switchgate/icon00";

            ninepatch_middle(
                entity,
                (24, 24),
                &frame,
                middle,
                asset_db,
                cx,
                r,
                map_pos,
                NinePatchOptions::default(),
            )?;
        }
        "goldenBlock" => ninepatch_middle(
            entity,
            (24, 24),
            "objects/goldblock",
            "collectables/goldberry/idle00",
            asset_db,
            cx,
            r,
            map_pos,
            NinePatchOptions::default(),
        )?,
        "templeCrackedBlock" => ninepatch_entity(
            entity,
            (24, 24),
            "objects/temple/breakBlock00",
            asset_db,
            cx,
            r,
            map_pos,
            NinePatchOptions::default(),
        )?,
        "templeMirror" => {
            ninepatch_entity(
                entity,
                (24, 24),
                "scenery/templemirror",
                asset_db,
                cx,
                r,
                map_pos,
                NinePatchOptions::default(),
            )?;
        }
        "crushBlock" => {
            let width = entity.raw.try_get_attr_int("width")?.unwrap_or(24);
            let height = entity.raw.try_get_attr_int("height")?.unwrap_or(24);

            let axes = entity.raw.try_get_attr("axes")?.unwrap_or("both");
            let chillout = entity.raw.try_get_attr("chillout")?.unwrap_or(false);

            let giant = height >= 48 && width >= 48 && chillout;

            let face_texture = match giant {
                true => "objects/crushblock/giant_block00",
                false => "objects/crushblock/idle_face",
            };

            let frame_texture = match axes {
                "none" => "objects/crushblock/block00",
                "horizontal" => "objects/crushblock/block01",
                "vertical" => "objects/crushblock/block02",
                "both" | _ => "objects/crushblock/block03",
            };

            nine_patch(
                asset_db,
                cx,
                r,
                frame_texture,
                map_pos,
                (width as i16, height as i16),
                NinePatchOptions::border(),
            )?;

            r.rect_inset(
                6.0,
                map_pos,
                (width as f32, height as f32),
                Color::from_rgba8(98, 34, 43, 255),
            );

            let face_sprite = asset_db.lookup_gameplay(cx, face_texture)?;
            r.sprite(
                cx,
                (
                    map_pos.0 + (width / 2) as f32,
                    map_pos.1 + (height / 2) as f32,
                ),
                face_sprite,
                SpriteDesc::default(),
            )?;
        }
        "moveBlock" => {
            let width = entity.raw.try_get_attr_int("width")?.unwrap_or(24);
            let height = entity.raw.try_get_attr_int("height")?.unwrap_or(24);

            let direction = entity
                .raw
                .try_get_attr("direction")?
                .unwrap_or("up")
                .to_lowercase();
            let can_steer = entity.raw.try_get_attr("canSteer")?.unwrap_or(false);
            let _buttons_on_side = matches!(direction.as_str(), "up" | "down");

            let block_texture = match (can_steer, direction.as_str()) {
                (true, "up") => "objects/moveBlock/base_v",
                (true, "left") => "objects/moveBlock/base_h",
                (true, "right") => "objects/moveBlock/base_h",
                (true, "down") => "objects/moveBlock/base_v",
                (false, _) | (true, _) => "objects/moveBlock/base",
            };

            let arrow_texture = match direction.as_str() {
                "left" => "objects/moveBlock/arrow04",
                "right" => "objects/moveBlock/arrow00",
                "down" => "objects/moveBlock/arrow06",
                "up" | _ => "objects/moveBlock/arrow02",
            };

            let highlight_color = Color::from_rgba8(71, 64, 112, 255);
            let _mid_color = Color::from_rgba8(4, 3, 23, 255);

            // highlight
            r.rect_inset(2.0, map_pos, (width as f32, height as f32), highlight_color);

            // TODO: reenable this, but the arrow texture is transparent so it doesn't look right
            /*let inset = 8.0;
            let (rect_x, rect_y) = r.transform_pos_f32((map_pos.0 + inset, map_pos.1 + inset));
            r.inset_rect(
                Rect::from_xywh(
                    rect_x,
                    rect_y,
                    width as f32 - (2. * inset),
                    height as f32 - (2. * inset),
                )
                .unwrap(),
                mid_color,
                BlendMode::default(),
            );*/

            nine_patch(
                asset_db,
                cx,
                r,
                block_texture,
                map_pos,
                (width as i16, height as i16),
                NinePatchOptions::border(),
            )?;

            let arrow_sprite = asset_db.lookup_gameplay(cx, arrow_texture)?;
            r.sprite(
                cx,
                (
                    map_pos.0 + (width / 2) as f32,
                    map_pos.1 + (height / 2) as f32,
                ),
                arrow_sprite,
                SpriteDesc::default(),
            )?;

            if can_steer {
                // TODO buttons
            }
        }
        "zipMover" => {
            let theme = entity
                .raw
                .try_get_attr("theme")?
                .unwrap_or("normal")
                .to_lowercase();

            let (block, light) = match theme.as_str() {
                "moon" => (
                    "objects/zipmover/moon/block",
                    "objects/zipmover/moon/light01",
                ),
                "normal" | _ => ("objects/zipmover/block", "objects/zipmover/light01"),
            };

            ninepatch_middle(
                entity,
                (16, 16),
                block,
                light,
                asset_db,
                cx,
                r,
                map_pos,
                NinePatchOptions::border(),
            )?;
        }
        "swapBlock" => {
            let theme = entity
                .raw
                .try_get_attr("theme")?
                .unwrap_or("normal")
                .to_lowercase();

            let (block, light) = match theme.as_str() {
                "moon" => (
                    "objects/swapblock/moon/blockRed",
                    "objects/swapblock/moon/midBlockRed00",
                ),
                "normal" | _ => (
                    "objects/swapblock/blockRed",
                    "objects/swapblock/midBlockRed00",
                ),
            };

            ninepatch_middle(
                entity,
                (8, 8),
                block,
                light,
                asset_db,
                cx,
                r,
                map_pos,
                NinePatchOptions::default(),
            )?;
        }
        // TODO(entity): swapBlock ninepatch
        "pandorasBox/coloredWater" => {
            let color = entity.raw.try_get_attr("color")?.unwrap_or("LightSkyBlue");
            let color = parse_color(color)?;

            let fill = Color::from_rgba(
                color.red() * 0.3,
                color.green() * 0.3,
                color.blue() * 0.3,
                0.6,
            )
            .unwrap();
            let border = Color::from_rgba(
                color.red() * 0.8,
                color.green() * 0.8,
                color.blue() * 0.8,
                0.8,
            )
            .unwrap();

            simple_outline(entity, r, map_pos, fill, border, BlendMode::default())?;
        }
        "summitcheckpoint" => {
            let number = entity.raw.try_get_attr_int("number")?.unwrap_or(0);
            let digit1 = number % 100 / 10;
            let digit2 = number % 10;

            let back = asset_db.lookup_gameplay(cx, "scenery/summitcheckpoints/base02")?;
            r.sprite(cx, map_pos, back, SpriteDesc::default())?;
            let back_digit1 = asset_db
                .lookup_gameplay(cx, &format!("scenery/summitcheckpoints/numberbg0{digit1}"))?;
            r.sprite(
                cx,
                (map_pos.0 - 2.0, map_pos.1 + 4.),
                back_digit1,
                SpriteDesc::default(),
            )?;
            let front_digit1 = asset_db
                .lookup_gameplay(cx, &format!("scenery/summitcheckpoints/number0{digit1}"))?;
            r.sprite(
                cx,
                (map_pos.0 - 2.0, map_pos.1 + 4.),
                front_digit1,
                SpriteDesc::default(),
            )?;
            let back_digit2 = asset_db
                .lookup_gameplay(cx, &format!("scenery/summitcheckpoints/numberbg0{digit2}"))?;
            r.sprite(
                cx,
                (map_pos.0 + 2.0, map_pos.1 + 4.),
                back_digit2,
                SpriteDesc::default(),
            )?;
            let front_digit2 = asset_db
                .lookup_gameplay(cx, &format!("scenery/summitcheckpoints/number0{digit2}"))?;
            r.sprite(
                cx,
                (map_pos.0 + 2.0, map_pos.1 + 4.),
                front_digit2,
                SpriteDesc::default(),
            )?;
        }
        _ => return Ok(false),
    }

    Ok(true)
}

fn ninepatch_entity<L: LookupAsset>(
    entity: &Entity,
    default_size: (i32, i32),
    texture_block: &str,
    asset_db: &mut AssetDb<L>,
    cx: &CelesteRenderData,
    r: &mut RenderContext<L>,
    map_pos: (f32, f32),
    options: NinePatchOptions,
) -> Result<(), anyhow::Error> {
    let width = entity
        .raw
        .try_get_attr_int("width")?
        .unwrap_or(default_size.0);
    let height = entity
        .raw
        .try_get_attr_int("height")?
        .unwrap_or(default_size.1);
    nine_patch(
        asset_db,
        cx,
        r,
        texture_block,
        map_pos,
        (width as i16, height as i16),
        options,
    )?;

    Ok(())
}

fn ninepatch_middle<L: LookupAsset>(
    entity: &Entity,
    default_size: (i32, i32),
    texture_block: &str,
    texture_middle: &str,
    asset_db: &mut AssetDb<L>,
    cx: &CelesteRenderData,
    r: &mut RenderContext<L>,
    map_pos: (f32, f32),
    options: NinePatchOptions,
) -> Result<(), anyhow::Error> {
    let width = entity
        .raw
        .try_get_attr_int("width")?
        .unwrap_or(default_size.0);
    let height = entity
        .raw
        .try_get_attr_int("height")?
        .unwrap_or(default_size.1);
    nine_patch(
        asset_db,
        cx,
        r,
        texture_block,
        map_pos,
        (width as i16, height as i16),
        options,
    )?;
    let middle_sprite = asset_db.lookup_gameplay(cx, texture_middle)?;
    r.sprite(
        cx,
        (
            map_pos.0 + (width / 2) as f32,
            map_pos.1 + (height / 2) as f32,
        ),
        middle_sprite,
        SpriteDesc::default(),
    )?;
    Ok(())
}

// bad substitute for depth
pub(super) fn pre_render_entity<L: LookupAsset>(
    r: &mut RenderContext<L>,
    cx: &CelesteRenderData,
    asset_db: &mut AssetDb<L>,
    room: &Room,
    entity: &Entity,
) -> Result<()> {
    match entity.name.as_str() {
        "spinner" => spinner_connectors(entity, room, asset_db, cx, r)?,
        _ => {}
    }

    Ok(())
}

fn get_spinner_texture(color: &str, foreground: bool) -> String {
    let prefix = if foreground { "fg_" } else { "bg_" };
    let texture = format!("danger/crystal/{prefix}{color}00");
    texture
}
fn spinner_connectors<L: LookupAsset>(
    entity: &Entity,
    room: &Room,
    asset_db: &mut AssetDb<L>,
    cx: &CelesteRenderData,
    r: &mut RenderContext<L>,
) -> Result<(), anyhow::Error> {
    let dust_override =
        r.area_id == Some(3) || (r.area_id == Some(7) && room.name.starts_with("lvl_d-"));

    let dust = entity
        .raw
        .try_get_attr::<bool>("dust")?
        .unwrap_or(dust_override);

    if dust {
        return Ok(());
    }

    let color_override = match r.area_id {
        Some(5) => Some("red"),
        Some(6) => Some("blue"),
        Some(10) => Some("rainbow"),
        _ => None,
    };

    let color = entity
        .raw
        .try_get_attr("color")?
        .or(color_override)
        .unwrap_or("blue")
        .to_ascii_lowercase();
    let color = match color.as_str() {
        "core" => "red",
        "rainbow" => "white",
        other => other,
    };
    let attach_to_solid = entity.raw.get_attr::<bool>("attachToSolid")?;
    for target in &room.entities {
        if target.id == entity.id {
            continue;
        }

        let target_dust = target
            .raw
            .try_get_attr::<bool>("dust")?
            .unwrap_or(dust_override);
        let target_attach_to_solid = entity.raw.get_attr::<bool>("attachToSolid")?;
        if entity.name == target.name && !target_dust && attach_to_solid == target_attach_to_solid {
            let delta_x = target.position.0 - entity.position.0;
            let delta_y = target.position.1 - entity.position.1;
            let dist_sq = delta_x * delta_x + delta_y * delta_y;
            if dist_sq < 24.0 * 24.0 {
                let connector_x = ((entity.position.0 + target.position.0) / 2.0).floor();
                let connector_y = ((entity.position.1 + target.position.1) / 2.0).floor();
                let sprite = get_spinner_texture(&color, false);
                let main_sprite = asset_db.lookup_gameplay(cx, &sprite)?;

                let connector_pos = room.bounds.position.offset_f32((connector_x, connector_y));
                r.sprite(cx, connector_pos, main_sprite, SpriteDesc::default())?;
            }
        }
    }
    Ok(())
}

fn spinner_main<L: LookupAsset>(
    entity: &Entity,
    room: &Room,
    asset_db: &mut AssetDb<L>,
    cx: &CelesteRenderData,
    r: &mut RenderContext<L>,
    map_pos: (f32, f32),
) -> Result<(), anyhow::Error> {
    let dust_override =
        r.area_id == Some(3) || (r.area_id == Some(7) && room.name.starts_with("lvl_d-"));

    let dust = entity
        .raw
        .try_get_attr::<bool>("dust")?
        .unwrap_or(dust_override);

    if dust {
        let base = asset_db.lookup_gameplay(cx, "danger/dustcreature/base00")?;
        r.sprite(
            cx,
            map_pos,
            base,
            SpriteDesc {
                ..Default::default()
            },
        )?;
        //let outline = asset_db.lookup_gameplay(cx, "@Internal@/dust_creature_outlines/base00")?;
        //r.sprite_new(cx,map_pos, outline, SpriteDesc {  justify: (0.5, 0.5),   ..Default::default() })?; // tint (1.0,0.0,0.0)
        return Ok(());
    }

    let color_override = match r.area_id {
        Some(5) => Some("red"),
        Some(6) => Some("blue"),
        Some(10) => Some("rainbow"),
        _ => None,
    };

    let color = entity
        .raw
        .try_get_attr("color")?
        .or(color_override)
        .unwrap_or("blue")
        .to_ascii_lowercase();
    let color = match color.as_str() {
        "core" => "red",
        "rainbow" => "white",
        other => other,
    };

    let main_sprite = get_spinner_texture(&color, true);
    let main_sprite = asset_db.lookup_gameplay(cx, &main_sprite)?;
    r.sprite(
        cx,
        map_pos,
        main_sprite,
        SpriteDesc {
            ..Default::default()
        },
    )?;
    Ok(())
}

fn simple_outline<L: LookupAsset>(
    entity: &Entity,
    r: &mut RenderContext<L>,
    map_pos: (f32, f32),
    color: Color,
    color_outline: Color,
    blend_mode: BlendMode,
) -> Result<(), anyhow::Error> {
    let width = entity.raw.get_attr_int("width")?;
    let height = entity.raw.get_attr_int("height")?;
    let (x, y) = r.transform_pos_f32(map_pos);
    let rect = Rect::from_xywh(x, y, width as f32, height as f32).unwrap();
    r.rect(rect, color, blend_mode);
    r.stroke_rect(rect, color_outline);
    Ok(())
}

#[derive(Clone, Copy)]
enum CardinalDir {
    Up,
    Down,
    Right,
    Left,
}

impl CardinalDir {
    fn as_str(self) -> &'static str {
        match self {
            CardinalDir::Up => "up",
            CardinalDir::Down => "down",
            CardinalDir::Left => "left",
            CardinalDir::Right => "right",
        }
    }

    fn horizontal(&self) -> bool {
        matches!(self, CardinalDir::Left | CardinalDir::Right)
    }

    fn orthogonal(self, (x, y): &mut (f32, f32)) -> &mut f32 {
        if self.horizontal() {
            y
        } else {
            x
        }
    }
}

fn spikes<L: LookupAsset>(
    map_pos: (f32, f32),
    entity: &Entity,
    dir: CardinalDir,
    trigger: bool, // TODO
    asset_db: &mut AssetDb<L>,
    cx: &CelesteRenderData,
    r: &mut RenderContext<L>,
) -> Result<()> {
    let variant = entity
        .raw
        .try_get_attr::<&str>("type")?
        .unwrap_or("default");
    let width = entity.raw.try_get_attr_int("width")?;
    let height = entity.raw.try_get_attr_int("height")?;

    if trigger {
        let length = match dir.horizontal() {
            true => entity.raw.try_get_attr_int("height")?,
            false => entity.raw.try_get_attr_int("width")?,
        }
        .unwrap_or(8);

        let (rotation_offset_x, rotation_offset_y) = match dir {
            CardinalDir::Up => (0, 0),
            CardinalDir::Down => (4, 0),
            CardinalDir::Right => (0, 0),
            CardinalDir::Left => (0, 4),
        };
        let (_second_offset_x, _second_offset_y) = match dir {
            CardinalDir::Up => (1, 0),
            CardinalDir::Down => (-1, 0),
            CardinalDir::Right => (0, 1),
            CardinalDir::Left => (0, -1),
        };
        // TODO. use rotation
        let _rotation = match dir {
            CardinalDir::Up => 0.0,
            CardinalDir::Down => PI,
            CardinalDir::Right => PI / 2.0,
            CardinalDir::Left => PI * 3.0 / 2.0,
        };

        for offset in (0..=length - 4).step_by(4) {
            let second_sprite = offset % 8 == 0;

            let offset_x = if dir.horizontal() { 0 } else { offset };
            let offset_y = if dir.horizontal() { offset } else { 0 };

            let colors = [
                Color::from_rgba8(242, 90, 16, 255),
                Color::from_rgba8(255, 0, 0, 255),
                Color::from_rgba8(242, 16, 103, 255),
            ];

            let color = fastrand::choice(colors).unwrap();

            let texture = match second_sprite {
                true => "danger/triggertentacle/wiggle_v03",
                false => "danger/triggertentacle/wiggle_v06",
            };
            let sprite = asset_db.lookup_gameplay(cx, texture)?;

            let pos = (
                map_pos.0 + offset_x as f32 + rotation_offset_x as f32,
                map_pos.1 + offset_y as f32 + rotation_offset_y as f32,
            );

            // TODO: this is a bit broken rn
            if second_sprite {
                // pos.0 += 4. * second_offset_x as f32 * (fastrand::f32() + 1.0);
                // pos.1 += 4. * second_offset_y as f32 * (fastrand::f32() + 1.0);
            }

            r.sprite(
                cx,
                pos,
                sprite,
                SpriteDesc {
                    justify: (0.0, 1.0),
                    tint: Some(color),
                    ..Default::default()
                },
            )?;
        }
        return Ok(());
    }

    let (texture, step) = match variant {
        "tentacles" => {
            let texture = format!("danger/tentacles00");
            (texture, 16)
        }
        "default" | "outline" | "cliffside" | "reflection" | _ => {
            let texture = format!("danger/spikes/{}_{}00", variant, dir.as_str());
            (texture, 8)
        }
    };

    let justification = match (variant, dir) {
        ("tentacles", CardinalDir::Up | CardinalDir::Right) => (0.0, 0.5),
        ("tentacles", CardinalDir::Down | CardinalDir::Left) => (1.0, 0.5),
        (_, CardinalDir::Up) => (0.5, 1.0),
        (_, CardinalDir::Down) => (0.5, 0.0),
        (_, CardinalDir::Right) => (0.0, 0.5),
        (_, CardinalDir::Left) => (1.0, 0.5),
    };

    let original_trigger = false; // TODO
    let offset = match (variant, dir, original_trigger) {
        (_, CardinalDir::Up, true) => (4, 5),
        (_, CardinalDir::Down, true) => (4, -5),
        (_, CardinalDir::Right, true) => (-5, 4),
        (_, CardinalDir::Left, true) => (5, 4),
        ("tentacles", CardinalDir::Up, false) => (0, 0),
        (_, CardinalDir::Up, false) => (4, 1),
        (_, CardinalDir::Down, false) => (4, -1),
        (_, CardinalDir::Right, false) => (-1, 4),
        (_, CardinalDir::Left, false) => (1, 4),
    };

    // TODO rotation
    let _rotation = match (variant, dir) {
        ("tentacles", CardinalDir::Up) => 0.0,
        ("tentacles", CardinalDir::Down) => PI,
        ("tentacles", CardinalDir::Right) => PI / 2.0,
        ("tentacles", CardinalDir::Left) => PI * 3.0 / 2.0,
        (_, _) => 0.0,
    };

    let length = match dir.horizontal() {
        true => height.unwrap_or(step),
        false => width.unwrap_or(step),
    };

    let mut position = map_pos;

    for i in (0..length).step_by(step as usize) {
        if i == length - step / 2 {
            *dir.orthogonal(&mut position) -= step as f32 / 2 as f32;
        }

        let sprite = asset_db.lookup_gameplay(cx, &texture)?;
        r.sprite(
            cx,
            (position.0 + offset.0 as f32, position.1 + offset.1 as f32),
            sprite,
            SpriteDesc {
                justify: justification,
                ..Default::default()
            },
        )?;

        *dir.orthogonal(&mut position) += step as f32;
    }

    Ok(())
}

fn jump_thru<L: LookupAsset>(
    entity: &Entity,
    fgtiles: &Matrix<char>,
    asset_db: &mut AssetDb<L>,
    cx: &CelesteRenderData,
    r: &mut RenderContext<L>,
    map_pos: (f32, f32),
) -> Result<()> {
    let texture_raw = entity
        .raw
        .try_get_attr::<&str>("texture")?
        .filter(|&texture| texture != "default")
        .unwrap_or("wood");
    let width = entity.raw.try_get_attr_num("width")?.unwrap_or(8.0);
    let (start_x, start_y) = (to_tile(entity.position.0), to_tile(entity.position.1));
    let stop_x = start_x + to_tile(width) - 1;
    let len = stop_x - start_x;
    Ok(for i in 0..len {
        let (mut quad_x, mut quad_y) = (8, 8);

        if i == 0 {
            quad_x = 0;
            quad_y = if fgtiles.get_or(start_x - 1, start_y, AIR) != AIR {
                0
            } else {
                8
            };
        } else if i == len - 1 {
            quad_y = if fgtiles.get_or(stop_x + 1, start_y, AIR) != AIR {
                0
            } else {
                8
            };
            quad_x = 16;
        }

        let sprite = asset_db.lookup_gameplay(cx, &format!("objects/jumpthru/{}", texture_raw))?;

        r.sprite(
            cx,
            (map_pos.0 + i as f32 * 8.0, map_pos.1),
            sprite,
            SpriteDesc {
                quad: Some((quad_x, quad_y, 8, 8)), // TODO: there's something leaking from the atlas at y=7,8
                ..Default::default()
            },
        )?;
    })
}

fn wire<L: LookupAsset>(room: &Room, entity: &Entity, r: &mut RenderContext<L>) -> Result<()> {
    let default_color = Color::from_rgba8(89, 88, 102, 255);
    let color = entity
        .raw
        .try_get_attr("color")?
        .map(parse_color)
        .transpose()?
        .unwrap_or(default_color);

    ensure!(
        entity.nodes.len() == 1,
        "wire has {} nodes instead of 1",
        entity.nodes.len()
    );

    let start = entity.position;
    let stop = entity.nodes[0].position;
    let control = ((start.0 + stop.0) / 2.0, (start.1 + stop.1) / 2.0 + 24.0);

    let resolution = 16;
    let curve = get_simple_curve(start, stop, control, resolution);

    let mut pb = PathBuilder::with_capacity(resolution as usize, resolution as usize);
    line(&mut pb, curve);

    /*let first = curve.next().unwrap();
    pb.move_to(first.0, first.1);
    for (x, y) in curve {
        pb.line_to(x, y);
    }*/

    // let start = curve.next().unwrap();
    /*pb.move_to(start.0, start.1);
    for (x, y) in curve {
        let (x, y) = r.transform_pos_f32((map_pos.0 + x, map_pos.1 + y));

        pb.line_to(x, y);
    }*/

    let room2img = Transform::from_translate(
        -r.map_bounds.position.x as f32 + room.bounds.position.x as f32,
        -r.map_bounds.position.y as f32 + room.bounds.position.y as f32,
    );

    r.pixmap.stroke_path(
        &pb.finish().unwrap(),
        &Paint {
            shader: tiny_skia::Shader::SolidColor(color),
            anti_alias: false,
            ..Default::default()
        },
        &Stroke::default(),
        room2img,
        None,
    );

    Ok(())
}

fn get_simple_curve(
    start: (f32, f32),
    stop: (f32, f32),
    control: (f32, f32),
    resolution: u32,
) -> impl Iterator<Item = (f32, f32)> {
    (0..=resolution).map(move |i| {
        let percent = i as f32 / resolution as f32;

        let start_mul = (1. - percent).powi(2);
        let control_mul = 2. * (1. - percent) * percent;
        let stop_mul = percent.powi(2);

        let x = start.0 * start_mul + control.0 * control_mul + stop.0 * stop_mul;
        let y = start.1 * start_mul + control.1 * control_mul + stop.1 * stop_mul;
        (x, y)
    })
}

fn line(pb: &mut PathBuilder, mut iter: impl Iterator<Item = (f32, f32)>) -> Option<()> {
    let first = iter.next()?;
    pb.move_to(first.0, first.1);
    for next in iter {
        pb.line_to(next.0, next.1);
    }

    Some(())
}

fn texture_map() -> &'static HashMap<&'static str, RenderMethod> {
    static TEX_MAP: OnceLock<HashMap<&'static str, RenderMethod>> = OnceLock::new();

    TEX_MAP.get_or_init(entity_impls::render_methods)
}

#[allow(dead_code)]
enum RenderMethod {
    Texture {
        texture: &'static str,
        justification: Option<(f32, f32)>,
        rotation: Option<f32>,
    },
    Rect {
        fill: Color,
        border: Color,
    },
    FakeTiles {
        material_key: &'static str,
        blend_key: bool,
        layer: Option<&'static str>,
        color: Option<Color>,
        x: Option<&'static str>,
        y: Option<&'static str>,
    },
}

fn parse_color(color: &str) -> Result<Color> {
    match color {
        "Transparent" => return Ok(Color::from_rgba8(0, 0, 0, 0)),
        "AliceBlue" => return Ok(Color::from_rgba8(240, 248, 255, 255)),
        "AntiqueWhite" => return Ok(Color::from_rgba8(250, 235, 215, 255)),
        "Aqua" => return Ok(Color::from_rgba8(0, 255, 255, 255)),
        "Aquamarine" => return Ok(Color::from_rgba8(127, 255, 212, 255)),
        "Azure" => return Ok(Color::from_rgba8(240, 255, 255, 255)),
        "Beige" => return Ok(Color::from_rgba8(245, 245, 220, 255)),
        "Bisque" => return Ok(Color::from_rgba8(255, 228, 196, 255)),
        "Black" => return Ok(Color::from_rgba8(0, 0, 0, 255)),
        "BlanchedAlmond" => return Ok(Color::from_rgba8(255, 235, 205, 255)),
        "Blue" => return Ok(Color::from_rgba8(0, 0, 255, 255)),
        "BlueViolet" => return Ok(Color::from_rgba8(138, 43, 226, 255)),
        "Brown" => return Ok(Color::from_rgba8(165, 42, 42, 255)),
        "BurlyWood" => return Ok(Color::from_rgba8(222, 184, 135, 255)),
        "CadetBlue" => return Ok(Color::from_rgba8(95, 158, 160, 255)),
        "Chartreuse" => return Ok(Color::from_rgba8(127, 255, 0, 255)),
        "Chocolate" => return Ok(Color::from_rgba8(210, 105, 30, 255)),
        "Coral" => return Ok(Color::from_rgba8(255, 127, 80, 255)),
        "CornflowerBlue" => return Ok(Color::from_rgba8(100, 149, 237, 255)),
        "Cornsilk" => return Ok(Color::from_rgba8(255, 248, 220, 255)),
        "Crimson" => return Ok(Color::from_rgba8(220, 20, 60, 255)),
        "Cyan" => return Ok(Color::from_rgba8(0, 255, 255, 255)),
        "DarkBlue" => return Ok(Color::from_rgba8(0, 0, 139, 255)),
        "DarkCyan" => return Ok(Color::from_rgba8(0, 139, 139, 255)),
        "DarkGoldenrod" => return Ok(Color::from_rgba8(184, 134, 11, 255)),
        "DarkGray" => return Ok(Color::from_rgba8(169, 169, 169, 255)),
        "DarkGreen" => return Ok(Color::from_rgba8(0, 100, 0, 255)),
        "DarkKhaki" => return Ok(Color::from_rgba8(189, 183, 107, 255)),
        "DarkMagenta" => return Ok(Color::from_rgba8(139, 0, 139, 255)),
        "DarkOliveGreen" => return Ok(Color::from_rgba8(85, 107, 47, 255)),
        "DarkOrange" => return Ok(Color::from_rgba8(255, 140, 0, 255)),
        "DarkOrchid" => return Ok(Color::from_rgba8(153, 50, 204, 255)),
        "DarkRed" => return Ok(Color::from_rgba8(139, 0, 0, 255)),
        "DarkSalmon" => return Ok(Color::from_rgba8(233, 150, 122, 255)),
        "DarkSeaGreen" => return Ok(Color::from_rgba8(143, 188, 139, 255)),
        "DarkSlateBlue" => return Ok(Color::from_rgba8(72, 61, 139, 255)),
        "DarkSlateGray" => return Ok(Color::from_rgba8(47, 79, 79, 255)),
        "DarkTurquoise" => return Ok(Color::from_rgba8(0, 206, 209, 255)),
        "DarkViolet" => return Ok(Color::from_rgba8(148, 0, 211, 255)),
        "DeepPink" => return Ok(Color::from_rgba8(255, 20, 147, 255)),
        "DeepSkyBlue" => return Ok(Color::from_rgba8(0, 191, 255, 255)),
        "DimGray" => return Ok(Color::from_rgba8(105, 105, 105, 255)),
        "DodgerBlue" => return Ok(Color::from_rgba8(30, 144, 255, 255)),
        "Firebrick" => return Ok(Color::from_rgba8(178, 34, 34, 255)),
        "FloralWhite" => return Ok(Color::from_rgba8(255, 250, 240, 255)),
        "ForestGreen" => return Ok(Color::from_rgba8(34, 139, 34, 255)),
        "Fuchsia" => return Ok(Color::from_rgba8(255, 0, 255, 255)),
        "Gainsboro" => return Ok(Color::from_rgba8(220, 220, 220, 255)),
        "GhostWhite" => return Ok(Color::from_rgba8(248, 248, 255, 255)),
        "Gold" => return Ok(Color::from_rgba8(255, 215, 0, 255)),
        "Goldenrod" => return Ok(Color::from_rgba8(218, 165, 32, 255)),
        "Gray" => return Ok(Color::from_rgba8(128, 128, 128, 255)),
        "Green" => return Ok(Color::from_rgba8(0, 128, 0, 255)),
        "GreenYellow" => return Ok(Color::from_rgba8(173, 255, 47, 255)),
        "Honeydew" => return Ok(Color::from_rgba8(240, 255, 240, 255)),
        "HotPink" => return Ok(Color::from_rgba8(255, 105, 180, 255)),
        "IndianRed" => return Ok(Color::from_rgba8(205, 92, 92, 255)),
        "Indigo" => return Ok(Color::from_rgba8(75, 0, 130, 255)),
        "Ivory" => return Ok(Color::from_rgba8(255, 255, 240, 255)),
        "Khaki" => return Ok(Color::from_rgba8(240, 230, 140, 255)),
        "Lavender" => return Ok(Color::from_rgba8(230, 230, 250, 255)),
        "LavenderBlush" => return Ok(Color::from_rgba8(255, 240, 245, 255)),
        "LawnGreen" => return Ok(Color::from_rgba8(124, 252, 0, 255)),
        "LemonChiffon" => return Ok(Color::from_rgba8(255, 250, 205, 255)),
        "LightBlue" => return Ok(Color::from_rgba8(173, 216, 230, 255)),
        "LightCoral" => return Ok(Color::from_rgba8(240, 128, 128, 255)),
        "LightCyan" => return Ok(Color::from_rgba8(224, 255, 255, 255)),
        "LightGoldenrodYellow" => return Ok(Color::from_rgba8(250, 250, 210, 255)),
        "LightGray" => return Ok(Color::from_rgba8(211, 211, 211, 255)),
        "LightGreen" => return Ok(Color::from_rgba8(144, 238, 144, 255)),
        "LightPink" => return Ok(Color::from_rgba8(255, 182, 193, 255)),
        "LightSalmon" => return Ok(Color::from_rgba8(255, 160, 122, 255)),
        "LightSeaGreen" => return Ok(Color::from_rgba8(32, 178, 170, 255)),
        "LightSkyBlue" => return Ok(Color::from_rgba8(135, 206, 250, 255)),
        "LightSlateGray" => return Ok(Color::from_rgba8(119, 136, 153, 255)),
        "LightSteelBlue" => return Ok(Color::from_rgba8(176, 196, 222, 255)),
        "LightYellow" => return Ok(Color::from_rgba8(255, 255, 224, 255)),
        "Lime" => return Ok(Color::from_rgba8(0, 255, 0, 255)),
        "LimeGreen" => return Ok(Color::from_rgba8(50, 205, 50, 255)),
        "Linen" => return Ok(Color::from_rgba8(250, 240, 230, 255)),
        "Magenta" => return Ok(Color::from_rgba8(255, 0, 255, 255)),
        "Maroon" => return Ok(Color::from_rgba8(128, 0, 0, 255)),
        "MediumAquamarine" => return Ok(Color::from_rgba8(102, 205, 170, 255)),
        "MediumBlue" => return Ok(Color::from_rgba8(0, 0, 205, 255)),
        "MediumOrchid" => return Ok(Color::from_rgba8(186, 85, 211, 255)),
        "MediumPurple" => return Ok(Color::from_rgba8(147, 112, 219, 255)),
        "MediumSeaGreen" => return Ok(Color::from_rgba8(60, 179, 113, 255)),
        "MediumSlateBlue" => return Ok(Color::from_rgba8(123, 104, 238, 255)),
        "MediumSpringGreen" => return Ok(Color::from_rgba8(0, 250, 154, 255)),
        "MediumTurquoise" => return Ok(Color::from_rgba8(72, 209, 204, 255)),
        "MediumVioletRed" => return Ok(Color::from_rgba8(199, 21, 133, 255)),
        "MidnightBlue" => return Ok(Color::from_rgba8(25, 25, 112, 255)),
        "MintCream" => return Ok(Color::from_rgba8(245, 255, 250, 255)),
        "MistyRose" => return Ok(Color::from_rgba8(255, 228, 225, 255)),
        "Moccasin" => return Ok(Color::from_rgba8(255, 228, 181, 255)),
        "NavajoWhite" => return Ok(Color::from_rgba8(255, 222, 173, 255)),
        "Navy" => return Ok(Color::from_rgba8(0, 0, 128, 255)),
        "OldLace" => return Ok(Color::from_rgba8(253, 245, 230, 255)),
        "Olive" => return Ok(Color::from_rgba8(128, 128, 0, 255)),
        "OliveDrab" => return Ok(Color::from_rgba8(107, 142, 35, 255)),
        "Orange" => return Ok(Color::from_rgba8(255, 165, 0, 255)),
        "OrangeRed" => return Ok(Color::from_rgba8(255, 69, 0, 255)),
        "Orchid" => return Ok(Color::from_rgba8(218, 112, 214, 255)),
        "PaleGoldenrod" => return Ok(Color::from_rgba8(238, 232, 170, 255)),
        "PaleGreen" => return Ok(Color::from_rgba8(152, 251, 152, 255)),
        "PaleTurquoise" => return Ok(Color::from_rgba8(175, 238, 238, 255)),
        "PaleVioletRed" => return Ok(Color::from_rgba8(219, 112, 147, 255)),
        "PapayaWhip" => return Ok(Color::from_rgba8(255, 239, 213, 255)),
        "PeachPuff" => return Ok(Color::from_rgba8(255, 218, 185, 255)),
        "Peru" => return Ok(Color::from_rgba8(205, 133, 63, 255)),
        "Pink" => return Ok(Color::from_rgba8(255, 192, 203, 255)),
        "Plum" => return Ok(Color::from_rgba8(221, 160, 221, 255)),
        "PowderBlue" => return Ok(Color::from_rgba8(176, 224, 230, 255)),
        "Purple" => return Ok(Color::from_rgba8(128, 0, 128, 255)),
        "Red" => return Ok(Color::from_rgba8(255, 0, 0, 255)),
        "RosyBrown" => return Ok(Color::from_rgba8(188, 143, 143, 255)),
        "RoyalBlue" => return Ok(Color::from_rgba8(65, 105, 225, 255)),
        "SaddleBrown" => return Ok(Color::from_rgba8(139, 69, 19, 255)),
        "Salmon" => return Ok(Color::from_rgba8(250, 128, 114, 255)),
        "SandyBrown" => return Ok(Color::from_rgba8(244, 164, 96, 255)),
        "SeaGreen" => return Ok(Color::from_rgba8(46, 139, 87, 255)),
        "SeaShell" => return Ok(Color::from_rgba8(255, 245, 238, 255)),
        "Sienna" => return Ok(Color::from_rgba8(160, 82, 45, 255)),
        "Silver" => return Ok(Color::from_rgba8(192, 192, 192, 255)),
        "SkyBlue" => return Ok(Color::from_rgba8(135, 206, 235, 255)),
        "SlateBlue" => return Ok(Color::from_rgba8(106, 90, 205, 255)),
        "SlateGray" => return Ok(Color::from_rgba8(112, 128, 144, 255)),
        "Snow" => return Ok(Color::from_rgba8(255, 250, 250, 255)),
        "SpringGreen" => return Ok(Color::from_rgba8(0, 255, 127, 255)),
        "SteelBlue" => return Ok(Color::from_rgba8(70, 130, 180, 255)),
        "Tan" => return Ok(Color::from_rgba8(210, 180, 140, 255)),
        "Teal" => return Ok(Color::from_rgba8(0, 128, 128, 255)),
        "Thistle" => return Ok(Color::from_rgba8(216, 191, 216, 255)),
        "Tomato" => return Ok(Color::from_rgba8(255, 99, 71, 255)),
        "Turquoise" => return Ok(Color::from_rgba8(64, 224, 208, 255)),
        "Violet" => return Ok(Color::from_rgba8(238, 130, 238, 255)),
        "Wheat" => return Ok(Color::from_rgba8(245, 222, 179, 255)),
        "White" => return Ok(Color::from_rgba8(255, 255, 255, 255)),
        "WhiteSmoke" => return Ok(Color::from_rgba8(245, 245, 245, 255)),
        "Yellow" => return Ok(Color::from_rgba8(255, 255, 0, 255)),
        "YellowGreen" => return Ok(Color::from_rgba8(154, 205, 50, 255)),
        _ => {}
    };

    ensure!(color.len() == 6, "unknown color: {color}");

    assert_eq!(color.len(), 6);

    let r = u8::from_str_radix(&color[..=1], 16)?;
    let g = u8::from_str_radix(&color[2..=3], 16)?;
    let b = u8::from_str_radix(&color[4..=5], 16)?;

    Ok(Color::from_rgba8(r, g, b, 255))
}
