use std::{borrow::Cow, collections::HashMap, f32::consts::PI, num::ParseIntError, sync::OnceLock};

use anyhow::{bail, ensure, Context, Result};
use celesteloader::map::{Entity, Room};
use tiny_skia::{Color, Paint, PathBuilder, Rect, Stroke, Transform};

use crate::{
    asset::{AssetDb, LookupAsset},
    rendering::AIR,
    CelesteRenderData,
};

use super::{Matrix, RenderContext};

fn parse_color(colors: &str) -> Result<Color, ParseIntError> {
    assert_eq!(colors.len(), 6);

    let r = u8::from_str_radix(&colors[..=1], 16)?;
    let g = u8::from_str_radix(&colors[2..=3], 16)?;
    let b = u8::from_str_radix(&colors[4..=5], 16)?;

    Ok(Color::from_rgba8(r, g, b, 255))
}

fn _coordinate_seed(x: f32, y: f32) -> u32 {
    // TODO make sure this is correct
    let shl = |x: f32, y: f32| x.to_bits() << y.to_bits();

    shl(x, f32::ceil(f32::log2(y.abs() + 1.0))) + y.abs() as u32
}

fn to_tile(val: f32) -> i32 {
    (val / 8.0).floor() as i32 + 1
}

pub(crate) fn render_entity<L: LookupAsset>(
    r: &mut RenderContext<L>,
    fgtiles: &Matrix<char>,
    cx: &CelesteRenderData,
    asset_db: &mut AssetDb<L>,
    room: &Room,
    entity: &Entity,
) -> Result<bool> {
    let map_pos = room.bounds.position.offset_f32(entity.position);

    let texture_map = texture_map();
    if let Some(texture) = texture_map.get(entity.name.as_str()) {
        let sprite = match asset_db
            .lookup_gameplay(cx, texture.texture)
            .context(entity.name.clone())
        {
            Ok(sprite) => sprite,
            Err(_) => return Ok(false),
        };
        r.sprite(
            cx,
            map_pos,
            (1.0, 1.0),
            texture.justification.unwrap_or((0.5, 0.5)),
            sprite,
            None,
            None,
        )?;
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
                (1.0, 1.0),
                (0.5, 1.0),
                asset,
                None,
                Some(color),
            )?;
        }
        "bird" => {
            let asset = asset_db.lookup_gameplay(cx, "characters/bird/crow00")?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 1.0), asset, None, None)?;
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
                (1.0, 1.0),
                (0.0, 0.0),
                sprite,
                Some((quad_x, 0, width, height)),
                None,
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
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 1.0), sprite, None, None)?;
        }
        "wire" => wire(room, entity, r)?,
        "refill" => {
            let two_dash = entity.raw.try_get_attr::<bool>("twoDash")?.unwrap_or(false);
            let texture = match two_dash {
                true => "objects/refillTwo/idle00",
                false => "objects/refill/idle00",
            };

            let sprite = asset_db.lookup_gameplay(cx, texture)?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 0.5), sprite, None, None)?;
        }
        "bonfire" => {
            let mode = entity.raw.try_get_attr::<&str>("mode")?.unwrap_or("lit");

            let texture = match mode {
                "lit" => "objects/campfire/fire08",
                "smoking" => "objects/campfire/smoking04",
                _ => "objects/campfire/fire00",
            };

            let sprite = asset_db.lookup_gameplay(cx, texture)?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 1.0), sprite, None, None)?;
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
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 0.5), sprite, None, None)?;

            for node in &entity.nodes {
                let pos = room
                    .bounds
                    .position
                    .offset_f32((node.position.0, node.position.1));
                let sprite = asset_db.lookup_gameplay(cx, "collectables/strawberry/seed00")?;
                r.sprite(cx, pos, (1.0, 1.0), (0.5, 0.5), sprite, None, None)?;
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
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 0.5), sprite, None, None)?;

            for node in &entity.nodes {
                let pos = room
                    .bounds
                    .position
                    .offset_f32((node.position.0, node.position.1));
                let sprite = asset_db.lookup_gameplay(cx, "collectables/goldberry/seed00")?;
                r.sprite(cx, pos, (1.0, 1.0), (0.5, 0.5), sprite, None, None)?;
            }
        }
        "blackGem" => {
            let sprite = asset_db.lookup_gameplay(cx, "collectables/heartGem/0/00")?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 0.5), sprite, None, None)?;
        }
        "cassette" => {
            let sprite = asset_db.lookup_gameplay(cx, "collectables/cassette/idle00")?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 0.5), sprite, None, None)?;
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
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 1.0), sprite, None, None)?;
        }
        "birdForsakenCityGem" => {
            let dish = asset_db.lookup_gameplay(cx, "objects/citysatellite/dish")?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 1.0), dish, None, None)?;

            let light = asset_db.lookup_gameplay(cx, "objects/citysatellite/light")?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 1.0), light, None, None)?;

            let computer_offset = (32.0, 24.0);
            let computer = asset_db.lookup_gameplay(cx, "objects/citysatellite/computer")?;
            r.sprite(
                cx,
                (map_pos.0 + computer_offset.0, map_pos.1 + computer_offset.1),
                (1.0, 1.0),
                (0.5, 0.5),
                computer,
                None,
                None,
            )?;
            let screen = asset_db.lookup_gameplay(cx, "objects/citysatellite/computerscreen")?;
            r.sprite(
                cx,
                (map_pos.0 + computer_offset.0, map_pos.1 + computer_offset.1),
                (1.0, 1.0),
                (0.5, 0.5),
                screen,
                None,
                None,
            )?;

            let mut nodes = entity.nodes.iter();
            let birds = nodes.next().context("satellite birds")?;
            let heart = nodes.next().context("satellite heart")?;
            let bird_pos = room.bounds.position.offset_f32(birds.position);
            let heart_pos = room.bounds.position.offset_f32(heart.position);

            let heart = asset_db.lookup_gameplay(cx, "collectables/heartGem/0/00")?;
            r.sprite(cx, heart_pos, (1.0, 1.0), (0.5, 0.5), heart, None, None)?;

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
                    (1.0, 1.0),
                    (0.5, 0.5),
                    light,
                    None,
                    Some(color),
                )?;
            }
        }
        "memorial" => {
            let sprite = asset_db.lookup_gameplay(cx, "scenery/memorial/memorial")?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 1.0), sprite, None, None)?;
        }
        "everest/memorial" => {
            let texture = entity
                .raw
                .try_get_attr::<&str>("sprite")?
                .unwrap_or("scenery/memorial/memorial");
            let sprite = asset_db.lookup_gameplay(cx, texture)?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 1.0), sprite, None, None)?;
        }
        "memorialTextController" => {
            let sprite = asset_db.lookup_gameplay(cx, "collectables/goldberry/wings01")?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 1.0), sprite, None, None)?;
        }
        "spring" => {
            let sprite = asset_db.lookup_gameplay(cx, "objects/spring/00")?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 1.0), sprite, None, None)?;
        }
        "booster" => {
            let red = entity.raw.try_get_attr("red")?.unwrap_or(false);

            if red {
                let sprite = asset_db.lookup_gameplay(cx, "objects/booster/boosterRed00")?;
                r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 1.0), sprite, None, None)?;
            } else {
                let sprite = asset_db.lookup_gameplay(cx, "objects/booster/booster00")?;
                r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 1.0), sprite, None, None)?;
            }
        }
        "cliffside_flag" => {
            let index = entity.raw.try_get_attr_int("index")?.unwrap_or(0);

            let sprite =
                asset_db.lookup_gameplay(cx, &format!("scenery/cliffside/flag{:02}", index))?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 1.0), sprite, None, None)?;
        }
        "torch" => {
            let fragile = entity.raw.get_attr("startLit")?;
            let texture = match fragile {
                true => "objects/temple/litTorch03",
                false => "objects/temple/torch00",
            };

            let sprite = asset_db.lookup_gameplay(cx, texture)?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 0.5), sprite, None, None)?;
        }
        "cloud" => {
            let fragile = entity.raw.get_attr("fragile")?;
            let texture = match fragile {
                true => "objects/clouds/fragile00",
                false => "objects/clouds/cloud00",
            };

            let sprite = asset_db.lookup_gameplay(cx, texture)?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 1.0), sprite, None, None)?;
        }
        "ridgeGate" => {
            let texture = entity
                .raw
                .try_get_attr::<&str>("texture")?
                .unwrap_or("objects/ridgeGate");
            let sprite = asset_db.lookup_gameplay(cx, texture)?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.0, 0.0), sprite, None, None)?;
        }
        "bigSpinner" => {
            let sprite = asset_db.lookup_gameplay(cx, "objects/Bumper/Idle22")?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 0.5), sprite, None, None)?;
        }
        "whiteblock" => {
            let sprite = asset_db.lookup_gameplay(cx, "objects/whiteblock")?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.0, 0.0), sprite, None, None)?;
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
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 1.0), sprite, None, None)?;
        }
        "payphone" => {
            let sprite = asset_db.lookup_gameplay(cx, "scenery/payphone")?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 1.0), sprite, None, None)?;
        }
        "towerviewer" => {
            let sprite = asset_db.lookup_gameplay(cx, "objects/lookout/lookout05")?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 1.0), sprite, None, None)?;
        }
        "key" => {
            let sprite = asset_db.lookup_gameplay(cx, "collectables/key/idle00")?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 1.0), sprite, None, None)?;
        }
        "infiniteStar" => {
            let shielded = entity.raw.try_get_attr("shielded")?.unwrap_or(false);

            let sprite = asset_db.lookup_gameplay(cx, "objects/flyFeather/idle00")?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 0.5), sprite, None, None)?;

            if shielded {
                r.circle(map_pos, 12.0, Color::WHITE);
            }
        }
        "touchSwitch" => {
            let container = asset_db.lookup_gameplay(cx, "objects/touchswitch/container")?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 0.5), container, None, None)?;
            let icon = asset_db.lookup_gameplay(cx, "objects/touchswitch/icon00")?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 0.5), icon, None, None)?;
        }
        "dreamBlock" => simple_outline(entity, r, map_pos, Color::BLACK, Color::WHITE)?,
        "invisibleBarrier" => {}
        "dreammirror" => {
            let frame = asset_db.lookup_gameplay(cx, "objects/mirror/frame")?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 1.0), frame, None, None)?;
            let glass = asset_db.lookup_gameplay(cx, "objects/mirror/glassbreak00")?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 1.0), glass, None, None)?;
        }
        "floatingDebris" => {
            let sprite = asset_db.lookup_gameplay(cx, "scenery/debris")?;
            let offset_x = fastrand::u8(0..7) * 8;

            r.sprite(
                cx,
                (map_pos.0 - 4.0, map_pos.1 - 4.0),
                (1.0, 1.0),
                (0.0, 0.0),
                sprite,
                Some((offset_x as i16, 0, 8, 8)),
                None,
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
                r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 0.5), sprite, None, None)?;
            }
        }
        "clutterCabinet" => {
            let sprite = asset_db.lookup_gameplay(cx, "objects/resortclutter/cabinet00")?;
            r.sprite(
                cx,
                (map_pos.0 + 8.0, map_pos.1 + 8.0),
                (1.0, 1.0),
                (0.5, 0.5),
                sprite,
                None,
                None,
            )?;
        }
        "colorSwitch" => {
            let variant = entity.raw.try_get_attr::<&str>("variant")?.unwrap_or("red");

            let button = asset_db.lookup_gameplay(cx, "objects/resortclutter/clutter_button00")?;
            r.sprite(
                cx,
                (map_pos.0 + 16.0, map_pos.1 + 16.0),
                (1.0, 1.0),
                (0.5, 1.0),
                button,
                None,
                None,
            )?;

            let clutter = asset_db.lookup_gameplay(
                cx,
                &format!("objects/resortclutter/icon_{}", variant.to_lowercase()),
            )?;
            r.sprite(
                cx,
                (map_pos.0 + 16.0, map_pos.1 + 8.0),
                (1.0, 1.0),
                (0.5, 0.5),
                clutter,
                None,
                None,
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
                (1.0, 1.0),
                (0.5, 0.5),
                sprite,
                None,
                None,
            )?;
        }
        "friendlyghost" => {
            let sprite = asset_db.lookup_gameplay(cx, "characteres/oshiro/boss13")?;
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 0.5), sprite, None, None)?;
        }
        "water" => {
            let light_blue = Color::from_rgba8(173, 216, 230, 255);
            let fill = Color::from_rgba(
                light_blue.red() * 0.3,
                light_blue.red() * 0.3,
                light_blue.red() * 0.3,
                0.6,
            )
            .unwrap();
            let border = Color::from_rgba(
                light_blue.red() * 0.8,
                light_blue.red() * 0.8,
                light_blue.red() * 0.8,
                0.8,
            )
            .unwrap();

            simple_outline(entity, r, map_pos, fill, border)?;
        }
        "spinner" => {
            spinner(entity, room, asset_db, cx, r, map_pos)?;
        }
        "fireBarrier" => {
            let color = Color::from_rgba8(209, 9, 1, 102);
            let color_outline = Color::from_rgba8(246, 98, 18, 255);
            simple_outline(entity, r, map_pos, color, color_outline)?;
        }
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
            r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 0.5), sprite, None, None)?;
        }
        "trackSpinner" => {
            let dust = entity.raw.try_get_attr::<bool>("dust")?.unwrap_or(false);
            let star = entity.raw.try_get_attr::<bool>("dust")?.unwrap_or(false);

            if star {
                let sprite = asset_db.lookup_gameplay(cx, "danger/starfish13")?;
                r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 0.5), sprite, None, None)?;
            } else if dust {
                let base = asset_db.lookup_gameplay(cx, "danger/dustcreature/base00")?;
                r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 0.5), base, None, None)?;
                let outline =
                    asset_db.lookup_gameplay(cx, "@Internal@/dust_creature_outlines/base00")?;
                r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 0.5), outline, None, None)?;
            } else {
                let sprite = asset_db.lookup_gameplay(cx, "danger/blade00")?;
                r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 0.5), sprite, None, None)?;
            }
        }
        _ => return Ok(false),
    }

    Ok(true)
}

fn spinner<L: LookupAsset>(
    entity: &Entity,
    room: &Room,
    asset_db: &mut AssetDb<L>,
    cx: &CelesteRenderData,
    r: &mut RenderContext<L>,
    map_pos: (f32, f32),
) -> Result<(), anyhow::Error> {
    let dusty = entity.raw.try_get_attr::<bool>("dust")?.unwrap_or(false);
    if dusty {
        todo!("dusty spinners");
    }
    let color = entity.raw.try_get_attr("color")?.unwrap_or("blue");
    let color = match color {
        "core" => "red",
        "rainbow" => "white",
        other => other,
    };
    let get_spinner_sprite = |color, foreground| {
        let prefix = if foreground { "fg_" } else { "bg_" };
        let texture = format!("danger/crystal/{prefix}{color}00");
        texture
    };
    let attach_to_solid = entity.raw.get_attr::<bool>("attachToSolid")?;
    for target in &room.entities {
        if target.id == entity.id {
            continue;
        }

        let target_dust = target.raw.try_get_attr::<bool>("dust")?.unwrap_or(false);
        let target_attach_to_solid = entity.raw.get_attr::<bool>("attachToSolid")?;
        if entity.name == target.name && !target_dust && attach_to_solid == target_attach_to_solid {
            let delta_x = target.position.0 - entity.position.0;
            let delta_y = target.position.1 - entity.position.1;
            let dist_sq = delta_x * delta_x + delta_y * delta_y;
            if dist_sq < 24.0 * 24.0 {
                let connector_x = ((entity.position.0 + target.position.0) / 2.0).floor();
                let connector_y = ((entity.position.1 + target.position.1) / 2.0).floor();
                let sprite = get_spinner_sprite(color, false);
                let main_sprite = asset_db.lookup_gameplay(cx, &sprite)?;

                let connector_pos = room.bounds.position.offset_f32((connector_x, connector_y));
                r.sprite(
                    cx,
                    connector_pos,
                    (1.0, 1.0),
                    (0.5, 0.5),
                    main_sprite,
                    None,
                    None,
                )?;
            }
        }
    }
    let main_sprite = get_spinner_sprite(color, true);
    let main_sprite = asset_db.lookup_gameplay(cx, &main_sprite)?;
    r.sprite(cx, map_pos, (1.0, 1.0), (0.5, 0.5), main_sprite, None, None)?;
    Ok(())
}

fn simple_outline<L: LookupAsset>(
    entity: &Entity,
    r: &mut RenderContext<L>,
    map_pos: (f32, f32),
    color: Color,
    color_outline: Color,
) -> Result<(), anyhow::Error> {
    let width = entity.raw.get_attr_int("width")?;
    let height = entity.raw.get_attr_int("height")?;
    let (x, y) = r.transform_pos_f32(map_pos);
    let rect = Rect::from_xywh(x, y, width as f32, height as f32).unwrap();
    r.rect(rect, color);
    r.stroke_rect(rect, color_outline);
    Ok(())
}

#[derive(Clone, Copy)]
enum CardinalDir {
    Up,
    Down,
    Left,
    Right,
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
    _trigger: bool, // TODO
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
            (1.0, 1.0),
            justification,
            sprite,
            None,
            None,
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
            (1.0, 1.0),
            (0.0, 0.0),
            sprite,
            Some((quad_x, quad_y, 8, 8)), // TODO: there's something leaking from the atlas at y=7,8
            None,
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

fn texture_map() -> &'static HashMap<&'static str, TextureDescription> {
    static TEX_MAP: OnceLock<HashMap<&'static str, TextureDescription>> = OnceLock::new();

    TEX_MAP.get_or_init(texture_map_init)
}

struct TextureDescription {
    texture: &'static str,
    justification: Option<(f32, f32)>,
}
#[rustfmt::skip]
fn texture_map_init() -> HashMap<&'static str, TextureDescription> {
    let mut textures = HashMap::new();

    textures.insert("CommunalHelper/SJ/AirTimeMusicController", TextureDescription { texture: "objects/CommunalHelper/strawberryJam/airTimeMusicController/icon", justification: None });
    textures.insert("VivHelper/DashBumper", TextureDescription { texture: "VivHelper/dashBumper/idle00", justification: None });
    textures.insert("FemtoHelper/BackdropWindController", TextureDescription { texture: "loenn/FemtoHelper/BackdropWindController", justification: Some((0.5, 0.5)) });   
    textures.insert("CollabUtils2/LobbyMapController", TextureDescription { texture: "CollabUtils2/editor_lobbymapmarker", justification: None });
    textures.insert("vitellary/interactivechaser", TextureDescription { texture: "characters/badeline/sleep00", justification: Some((0.5, 1.0)) });
    textures.insert("AuraHelper/IceKiller", TextureDescription { texture: "objects/icekiller", justification: None });
    textures.insert("AurorasHelper/FlagDirectionGem", TextureDescription { texture: "objects/reflectionHeart/gem", justification: Some((0.5, 0.5)) });
    textures.insert("CollabUtils2/SilverBerry", TextureDescription { texture: "CollabUtils2/silverBerry/idle00", justification: None });
    textures.insert("XaphanHelper/InGameMapHintController", TextureDescription { texture: "util/XaphanHelper/Loenn/hintController", justification: None });
    textures.insert("CommunalHelper/SJ/BulletTimeController", TextureDescription { texture: "objects/CommunalHelper/strawberryJam/bulletTimeController/icon", justification: None });
    textures.insert("MaxHelpingHand/SeekerBarrierColorControllerDisabler", TextureDescription { texture: "ahorn/MaxHelpingHand/rainbowSpinnerColorControllerDisable", justification: None });
    textures.insert("MemorialHelper/FlagCrystalHeart", TextureDescription { texture: "collectables/heartGem/white00", justification: None });
    textures.insert("ChronoHelper/LavaSwitch", TextureDescription { texture: "objects/chronohelper/lavaSwitch/switch_0.png", justification: None });
    textures.insert("EeveeHelper/LenientCeilingPopController", TextureDescription { texture: "objects/EeveeHelper/lenientCeilingPopController/icon", justification: None });
    textures.insert("VivHelper/EnergyCrystal", TextureDescription { texture: "VivHelper/entities/gem", justification: None });
    textures.insert("DJMapHelper/shield", TextureDescription { texture: "objects/DJMapHelper/shield/shield", justification: None });
    textures.insert("VortexHelper/BowlPuffer", TextureDescription { texture: "objects/VortexHelper/pufferBowl/idle00", justification: None });
    textures.insert("VivHelper/HideRoomInMap", TextureDescription { texture: "ahorn/VivHelper/HiddenRoom", justification: None });
    textures.insert("MaxHelpingHand/BeeFireball", TextureDescription { texture: "objects/MaxHelpingHand/beeFireball/beefireball00", justification: None });
    textures.insert("MaxHelpingHand/ParallaxFadeSpeedController", TextureDescription { texture: "@Internal@/northern_lights", justification: None });
    textures.insert("CommunalHelper/SJ/FlagBreakerBox", TextureDescription { texture: "objects/breakerBox/Idle00", justification: None });
    textures.insert("JungleHelper/CassetteCustomPreviewMusic", TextureDescription { texture: "collectables/cassette/idle00", justification: None });
    textures.insert("PlatinumStrawberry/PlatinumBadelineBoost", TextureDescription { texture: "objects/badelineboost/idle00", justification: None });
    textures.insert("GameHelper/PlayerStateFlag", TextureDescription { texture: "loenn/GameHelper/flag_controller", justification: Some((0.0, 0.0)) });
    textures.insert("ArphimigonHelper/TempleEyeball", TextureDescription { texture: "danger/templeeye/body00", justification: Some((0.5, 0.5)) });
    textures.insert("ArphimigonHelper/HeartOfTheStormContainer", TextureDescription { texture: "objects/crystalHeartContainer/empty", justification: Some((0.5, 0.5)) }); 
    textures.insert("AurorasHelper/DieOnFlagsController", TextureDescription { texture: "controllers/AurorasHelper/DieOnFlagsController", justification: Some((0.5, 1.0)) 
    });
    textures.insert("DJMapHelper/badelineBoostTeleport", TextureDescription { texture: "objects/badelineboost/idle00", justification: None });
    textures.insert("MaxHelpingHand/RainbowSpinnerColorControllerDisabler", TextureDescription { texture: "ahorn/MaxHelpingHand/rainbowSpinnerColorControllerDisable", justification: None });
    textures.insert("ArphimigonHelper/GiantClam", TextureDescription { texture: "objects/giantClam/open100", justification: Some((0.0, 1.0)) });
    textures.insert("AurorasHelper/FriendlySeeker", TextureDescription { texture: "characters/monsters/predator73", justification: None });
    textures.insert("FemtoHelper/CustomMoonCreature", TextureDescription { texture: "scenery/moon_creatures/tiny01", justification: None });
    textures.insert("FlaglinesAndSuch/BloomedOshiro", TextureDescription { texture: "objects/FlaglinesAndSuch/bloomedoshiro/boss13", justification: None });
    textures.insert("CommunalHelper/SJ/ExpiringDashRefill", TextureDescription { texture: "objects/refill/idle00", justification: None });
    textures.insert("PlatinumStrawberry/PlatinumStrawberry", TextureDescription { texture: "SyrenyxPlatinumStrawberry/collectables/platinumberry/plat00", justification: None });
    textures.insert("pandorasBox/waterDrowningController", TextureDescription { texture: "objects/pandorasBox/controllerIcons/waterDrowningController", justification: None });
    textures.insert("pandorasBox/dustSpriteColorController", TextureDescription { texture: "objects/pandorasBox/controllerIcons/dustSpriteColorController", justification: None });
    textures.insert("batteries/power_refill", TextureDescription { texture: "batteries/power_refill/idle00", justification: None });
    textures.insert("CollabUtils2/RainbowBerry", TextureDescription { texture: "CollabUtils2/rainbowBerry/rberry0030", justification: None });
    textures.insert("CommunalHelper/GlowController", TextureDescription { texture: "objects/CommunalHelper/glowController/icon", justification: None });
    textures.insert("XaphanHelper/HeatController", TextureDescription { texture: "util/XaphanHelper/Loenn/heatController", justification: None });
    textures.insert("Galactica/BlackHole", TextureDescription { texture: "BlackHole/Blackhole00", justification: None });
    textures.insert("vitellary/custompuffer", TextureDescription { texture: "objects/puffer/idle00", justification: None });
    textures.insert("vitellary/cassetteflags", TextureDescription { texture: "CrystallineHelper/FLCC/ahorn_cassetteflagcontroller", justification: None });
    textures.insert("AuraHelper/Health", TextureDescription { texture: "objects/health", justification: None });
    textures.insert("corkr900CoopHelper/GroupButton", TextureDescription { texture: "corkr900/CoopHelper/GroupSwitch/button00", justification: None });
    textures.insert("GameHelper/DashMagnet", TextureDescription { texture: "objects/GameHelper/dash_magnet/idle1", justification: Some((0.0, 0.0)) });
    textures.insert("MaxHelpingHand/CustomizableGlassBlockController", TextureDescription { texture: "@Internal@/northern_lights", justification: None });
    textures.insert("MaxHelpingHand/StaticPuffer", TextureDescription { texture: "objects/puffer/idle00", justification: None });
    textures.insert("VivHelper/RedDashRefill", TextureDescription { texture: "VivHelper/redDashRefill/redIdle00", justification: None });
    textures.insert("VivHelper/CustomPlaybackWatchtower", TextureDescription { texture: "objects/lookout/lookout05", justification: Some((0.5, 1.0)) });
    textures.insert("CommunalHelper/ResetStateCrystal", TextureDescription { texture: "objects/CommunalHelper/resetStateCrystal/ghostIdle00", justification: None });     
    textures.insert("VivHelper/OrangeBooster", TextureDescription { texture: "VivHelper/boosters/boosterOrange00", justification: None });
    textures.insert("cpopBlock", TextureDescription { texture: "cpopBlock", justification: Some((0.0, 0.0)) });
    textures.insert("FactoryHelper/BatteryBox", TextureDescription { texture: "objects/FactoryHelper/batteryBox/inactive0", justification: None });
    textures.insert("VivHelper/RefilllessBumper", TextureDescription { texture: "ahorn/VivHelper/norefillBumper", justification: None });
    textures.insert("ShroomHelper/OneDashWingedStrawberry", TextureDescription { texture: "collectables/ghostgoldberry/wings01", justification: None });
    textures.insert("Anonhelper/SuperDashRefill", TextureDescription { texture: "objects/AnonHelper/superDashRefill/idle00", justification: None });
    textures.insert("GameHelper/PushBoxButton", TextureDescription { texture: "objects/GameHelper/push_box_button/idle", justification: Some((0.0, 0.0)) });
    textures.insert("vitellary/roomname", TextureDescription { texture: "ahorn_roomname", justification: None });
    textures.insert("CherryHelper/ShadowBumper", TextureDescription { texture: "objects/shadowBumper/shadow22", justification: None });
    textures.insert("MaxHelpingHand/FlagRainbowSpinnerColorController", TextureDescription { texture: "@Internal@/northern_lights", justification: None });
    textures.insert("CommunalHelper/BadelineBoostKeepHoldables", TextureDescription { texture: "objects/badelineboost/idle00", justification: None });
    textures.insert("AurorasHelper/InternetMemorial", TextureDescription { texture: "scenery/memorial/memorial", justification: Some((0.5, 1.0)) });
    textures.insert("corkr900CoopHelper/SessionPicker", TextureDescription { texture: "corkr900/CoopHelper/SessionPicker/idle00", justification: None });
    textures.insert("MaxHelpingHand/CustomizableBerry", TextureDescription { texture: "collectables/strawberry/normal00", justification: None });
    textures.insert("AdventureHelper/BladeTrackSpinnerMultinode", TextureDescription { texture: "danger/blade00", justification: None });
    textures.insert("FemtoHelper/AssistHazardController", TextureDescription { texture: "loenn/FemtoHelper/squishcontroller", justification: None });
    textures.insert("GameHelper/Trampoline", TextureDescription { texture: "objects/GameHelper/trampoline/idle", justification: None });
    textures.insert("EeveeHelper/NoDemoBindController", TextureDescription { texture: "objects/EeveeHelper/noDemoBindController/icon", justification: None });
    textures.insert("ReverseHelper/ZiplineZipmover", TextureDescription { texture: "isafriend/objects/zipline/handle", justification: None });
    textures.insert("SaladimHelper/BitsMagicLanternController", TextureDescription { texture: "SaladimHelper/entities/bitsMagicLantern/controller", justification: None });
    textures.insert("FlaglinesAndSuch/DustNoShrinkController", TextureDescription { texture: "ahorn/FlaglinesAndSuch/dust_no_shrink", justification: None });
    textures.insert("SSMHelper/ZeroGravBoundsController", TextureDescription { texture: "loenn/SSMHelper/zerogravcontroller", justification: None });
    textures.insert("VivHelper/BumperWrapper", TextureDescription { texture: "ahorn/VivHelper/bumperWrapper", justification: None });
    textures.insert("CommunalHelper/CassetteJumpFixController", TextureDescription { texture: "objects/CommunalHelper/cassetteJumpFixController/icon", justification: None });
    textures.insert("ArphimigonHelper/ElementalRuneTablet", TextureDescription { texture: "objects/lookout/lookout05", justification: Some((0.5, 1.0)) });
    textures.insert("ChronoHelper/ShatterRefill", TextureDescription { texture: "objects/chronohelper/destroyRefill/idle00", justification: None });
    textures.insert("EeveeHelper/CoreZoneStartController", TextureDescription { texture: "objects/EeveeHelper/coreZoneStartController/icon", justification: None });      
    textures.insert("SSMHelper/CrystalBombBadelineBoss", TextureDescription { texture: "objects/SSMHelper/crystalBombBadelineBoss/charge00", justification: None });      
    textures.insert("MaxHelpingHand/CustomCh3MemoOnFlagController", TextureDescription { texture: "ahorn/MaxHelpingHand/set_flag_on_spawn", justification: None });       
    textures.insert("MaxHelpingHand/FlagDecalXML", TextureDescription { texture: "ahorn/MaxHelpingHand/flag_decal_xml", justification: None });
    textures.insert("CollabUtils2/LobbyMapMarker", TextureDescription { texture: "CollabUtils2/editor_lobbymapmarker", justification: None });
    textures.insert("XaphanHelper/TimedStrawberry", TextureDescription { texture: "collectables/strawberry/normal00", justification: None });
    textures.insert("XaphanHelper/InGameMapRoomController", TextureDescription { texture: "util/XaphanHelper/Loenn/roomController", justification: None });
    textures.insert("ArphimigonHelper/ThrowableRefillContainer", TextureDescription { texture: "objects/throwableRefillContainer/idle00", justification: Some((0.5, 0.5)) 
    });
    textures.insert("MaxHelpingHand/CustomTutorialWithNoBird", TextureDescription { texture: "ahorn/MaxHelpingHand/greyscale_birb", justification: Some((0.5, 1.0)) });   
    textures.insert("VivHelper/WarpDashRefill", TextureDescription { texture: "VivHelper/TSStelerefill/idle00", justification: None });
    textures.insert("YetAnotherHelper/StickyJellyfish", TextureDescription { texture: "ahorn/YetAnotherHelper/stickyJellyfish", justification: None });
    textures.insert("SaladimHelper/CustomAscendManager", TextureDescription { texture: "@Internal@/summit_background_manager", justification: None });
    textures.insert("SSMHelper/DelayedUltraIndicatorController", TextureDescription { texture: "loenn/SSMHelper/dultraindicatorcontroller", justification: None });       
    textures.insert("corkr900CoopHelper/SyncedSummitBackgroundManager", TextureDescription { texture: "@Internal@/summit_background_manager", justification: None });     
    textures.insert("DJMapHelper/flingBirdReversed", TextureDescription { texture: "characters/bird/Hover04", justification: None });
    textures.insert("MaxHelpingHand/SetFlagOnHeartCollectedController", TextureDescription { texture: "ahorn/MaxHelpingHand/set_flag_on_spawn", justification: None });   
    textures.insert("GameHelper/FlagCollectBerry", TextureDescription { texture: "collectables/strawberry/normal00", justification: None });
    textures.insert("BrokemiaHelper/dashSpringDown", TextureDescription { texture: "objects/BrokemiaHelper/dashSpring/00", justification: Some((0.5, 1.0)) });
    textures.insert("corkr900CoopHelper/ForceInteractionsController", TextureDescription { texture: "corkr900/CoopHelper/InteractionsController/icon00", justification: None });
    textures.insert("DJMapHelper/oshiroBossRight", TextureDescription { texture: "characters/oshiro/boss13", justification: None });
    textures.insert("TheoJelly", TextureDescription { texture: "objects/TheoJelly/idle0", justification: None });
    textures.insert("CommunalHelper/ManualCassetteController", TextureDescription { texture: "objects/CommunalHelper/manualCassetteController/icon", justification: None });
    textures.insert("JungleHelper/BreakablePot", TextureDescription { texture: "JungleHelper/Breakable Pot/breakpotidle", justification: None });
    textures.insert("JungleHelper/Firefly", TextureDescription { texture: "JungleHelper/Firefly/firefly00", justification: None });
    textures.insert("ChronoHelper/BoomBooster", TextureDescription { texture: "objects/chronohelper/boomBooster/booster00", justification: None });
    textures.insert("vitellary/boostbumper", TextureDescription { texture: "objects/boostBumper/booster00", justification: None });
    textures.insert("JungleHelper/Snake", TextureDescription { texture: "JungleHelper/Snake/IdleAggro/snake_idle00", justification: None });
    textures.insert("Anonhelper/FeatherBumper", TextureDescription { texture: "objects/AnonHelper/featherBumper/Idle22", justification: None });
    textures.insert("BrokemiaHelper/dashSpring", TextureDescription { texture: "objects/BrokemiaHelper/dashSpring/00", justification: Some((0.5, 1.0)) });
    textures.insert("ArphimigonHelper/SnappingClam", TextureDescription { texture: "objects/snappingClam/idle00", justification: Some((0.5, 0.5)) });
    textures.insert("Galactica/StarLight", TextureDescription { texture: "StarLight/StarLight00", justification: None });
    textures.insert("batteries/recharge_platform", TextureDescription { texture: "batteries/recharge_platform/base0", justification: Some((0.5, 1.0)) });
    textures.insert("CollabUtils2/GoldenBerryPlayerRespawnPoint", TextureDescription { texture: "characters/player/sitDown00", justification: Some((0.5, 1.0)) });        
    textures.insert("MaxHelpingHand/HorizontalRoomWrapController", TextureDescription { texture: "ahorn/MaxHelpingHand/horizontal_room_wrap", justification: None });     
    textures.insert("MaxHelpingHand/StylegroundFadeController", TextureDescription { texture: "@Internal@/northern_lights", justification: None });
    textures.insert("XaphanHelper/InGameMapSubAreaController", TextureDescription { texture: "util/XaphanHelper/Loenn/subAreaController", justification: None });
    textures.insert("ArphimigonHelper/HeartGem", TextureDescription { texture: "collectables/heartGem/3/00", justification: Some((0.5, 0.5)) });
    textures.insert("GameHelper/DecalMover", TextureDescription { texture: "loenn/GameHelper/decal_mover", justification: Some((0.0, 0.0)) });
    textures.insert("MaxHelpingHand/SetFlagOnCompletionController", TextureDescription { texture: "ahorn/MaxHelpingHand/set_flag_on_spawn", justification: None });       
    textures.insert("XaphanHelper/TimerRefill", TextureDescription { texture: "objects/XaphanHelper/TimerRefill/idle00", justification: None });
    textures.insert("JungleHelper/EnforceSkinController", TextureDescription { texture: "ahorn/JungleHelper/enforce_skin_controller", justification: None });
    textures.insert("MaxHelpingHand/GoldenStrawberryCustomConditions", TextureDescription { texture: "collectables/goldberry/idle00", justification: None });
    textures.insert("ArphimigonHelper/CoreMessage", TextureDescription { texture: "@Internal@/core_message", justification: None });
    textures.insert("MaxHelpingHand/RainbowSpinnerColorController", TextureDescription { texture: "@Internal@/northern_lights", justification: None });
    textures.insert("canyon/spinorb", TextureDescription { texture: "objects/canyon/spinorb/idle00", justification: Some((0.5, 0.5)) });
    textures.insert("CommunalHelper/DreamBoosterAny", TextureDescription { texture: "objects/CommunalHelper/boosters/dreamBooster/idle00", justification: None });        
    textures.insert("MaxHelpingHand/CustomNPCSprite", TextureDescription { texture: "ahorn/MaxHelpingHand/custom_npc_xml", justification: Some((0.5, 1.0)) });
    textures.insert("vitellary/dashcodecontroller", TextureDescription { texture: "ahorn_dashcodecontroller", justification: None });
    textures.insert("JungleHelper/Torch", TextureDescription { texture: "JungleHelper/TorchNight/TorchNightOff", justification: None });
    textures.insert("JungleHelper/TheoStatue", TextureDescription { texture: "JungleHelper/TheoStatue/idle00", justification: None });
    textures.insert("CherryHelper/EntityToggleBell", TextureDescription { texture: "objects/itemToggleBell/bell00", justification: Some((0.5, 0.5)) });
    textures.insert("AuraHelper/Fire", TextureDescription { texture: "objects/fire2", justification: None });
    textures.insert("CherryHelper/RottenBerry", TextureDescription { texture: "collectables/rottenberry/normal00", justification: Some((0.5, 0.5)) });
    textures.insert("AuraHelper/Bird", TextureDescription { texture: "objects/bird1", justification: None });
    textures.insert("CommunalHelper/UnderwaterMusicController", TextureDescription { texture: "objects/CommunalHelper/underwaterMusicController/icon", justification: None });
    textures.insert("AurorasHelper/FairySpawner", TextureDescription { texture: "objects/aurora_aquir/fairy_spawner/portal", justification: Some((0.5, 0.5)) });
    textures.insert("CherryHelper/BadelineBot", TextureDescription { texture: "characters/player_badeline/sitDown00", justification: Some((0.5, 1.0)) });
    textures.insert("FlaglinesAndSuch/StandBox", TextureDescription { texture: "objects/FlaglinesAndSuch/standbox/idle00", justification: None });
    textures.insert("FlaglinesAndSuch/BonfireLight", TextureDescription { texture: "ahorn/FlaglinesAndSuch/bonfireIcon", justification: Some((0.0, 0.0)) });
    textures.insert("DSidesPlatinum/HiddenStrawberry", TextureDescription { texture: "collectables/ghostberry/idle00", justification: None });
    textures.insert("VivHelper/DebrisLimiter", TextureDescription { texture: "ahorn/VivHelper/DebrisLimiter", justification: None });
    textures.insert("XaphanHelper/InGameMapRoomAdjustController", TextureDescription { texture: "util/XaphanHelper/Loenn/roomAdjustController", justification: None });   
    textures.insert("JungleHelper/RemoteKevinRefill", TextureDescription { texture: "JungleHelper/SlideBlockRefill/idle00", justification: None });
    textures.insert("AurorasHelper/ChangeRespawnOrb", TextureDescription { texture: "objects/respawn_orb/idle00", justification: None });
    textures.insert("VivHelper/PinkBooster", TextureDescription { texture: "VivHelper/boosters/boosterPink00", justification: None });
    textures.insert("SSMHelper/ForceCassetteBlockController", TextureDescription { texture: "loenn/SSMHelper/forcecassetteblockcontroller", justification: None });       
    textures.insert("FactoryHelper/DashFuseBox", TextureDescription { texture: "objects/FactoryHelper/dashFuseBox/idle00", justification: Some((0.0, 0.0)) });
    textures.insert("CollabUtils2/WarpPedestal", TextureDescription { texture: "CollabUtils2/placeholderorb/placeholderorb00", justification: Some((0.5, 0.95)) });       
    textures.insert("JungleHelper/AttachTriggerController", TextureDescription { texture: "ahorn/JungleHelper/attach_trigger_trigger", justification: Some((0.0, 0.0)) });textures.insert("CNY2024Helper/EasingBlackhole", TextureDescription { texture: "decals/ChineseNewYear2024/StarSapphire/GDDNblackhole/asmallblackholecanrotitself00", justification: Some((0.5, 0.5)) });
    textures.insert("GlitchHelper/Glitch", TextureDescription { texture: "objects/glitch/glitchgreen00", justification: None });
    textures.insert("Anonhelper/JellyRefill", TextureDescription { texture: "objects/AnonHelper/jellyRefill/idle00", justification: None });
    textures.insert("GameHelper/SuperHotController", TextureDescription { texture: "loenn/GameHelper/super_hot_controller", justification: Some((0.0, 0.0)) });
    textures.insert("ArphimigonHelper/AnchoredSpinnerParent", TextureDescription { texture: "danger/dustcreature/center00", justification: Some((0.5, 0.5)) });
    textures.insert("BrokemiaHelper/wallDashSpringRight", TextureDescription { texture: "objects/BrokemiaHelper/dashSpring/00", justification: Some((0.5, 1.0)) });       
    textures.insert("FrostHelper/KeyIce", TextureDescription { texture: "collectables/FrostHelper/keyice/idle00", justification: None });
    textures.insert("quizController", TextureDescription { texture: "quizController", justification: Some((0.0, 0.0)) });
    textures.insert("VivHelper/EvilBumper", TextureDescription { texture: "objects/Bumper/Evil22", justification: None });
    textures.insert("pandorasBox/laserEmitter", TextureDescription { texture: "objects/pandorasBox/laser/emitter/idle0", justification: Some((0.5, 1.0)) });
    textures.insert("AurorasHelper/HorizontalCollisionDeathController", TextureDescription { texture: "controllers/AurorasHelper/HorizontalCollisionDeathController", justification: Some((0.5, 1.0)) });
    textures.insert("AurorasHelper/BulletHellController", TextureDescription { texture: "controllers/AurorasHelper/BulletHellController", justification: None });
    textures.insert("CommunalHelper/LightningController", TextureDescription { texture: "objects/CommunalHelper/lightningController/icon", justification: None });        
    textures.insert("CherryHelper/AnterogradeController", TextureDescription { texture: "objects/anterogradeController/icon", justification: Some((0.5, 0.5)) });
    textures.insert("ExtendedVariantMode/VariantToggleController", TextureDescription { texture: "ahorn/ExtendedVariantMode/whydrawarectanglewhenyoucandrawapngofarectangleinstead", justification: Some((0.0, 0.0)) });
    textures.insert("MaxHelpingHand/SetFlagOnButtonPressController", TextureDescription { texture: "ahorn/MaxHelpingHand/set_flag_on_button", justification: None });
    textures.insert("GlitchHelper/BlueGlitch", TextureDescription { texture: "objects/glitch/glitchblue00", justification: None });
    textures.insert("DJMapHelper/badelineBoostDown", TextureDescription { texture: "objects/badelineboost/idle00", justification: None });
    textures.insert("MaxHelpingHand/SecretBerry", TextureDescription { texture: "collectables/moonBerry/normal00", justification: None });
    textures.insert("pandorasBox/airBubbles", TextureDescription { texture: "objects/pandorasBox/airBubbles/idle00", justification: None });
    textures.insert("VivHelper/PreviousBerriesToFlag", TextureDescription { texture: "ahorn/VivHelper/PrevBerriesToFlag", justification: None });
    textures.insert("vitellary/fillcrystal", TextureDescription { texture: "objects/crystals/fill/idle00", justification: None });
    textures.insert("Anonhelper/FeatherRefill", TextureDescription { texture: "objects/AnonHelper/featherRefill/idle00", justification: None });
    textures.insert("MaxHelpingHand/SeekerBarrierColorController", TextureDescription { texture: "@Internal@/northern_lights", justification: None });
    textures.insert("MaxHelpingHand/SetFlagOnActionController", TextureDescription { texture: "ahorn/MaxHelpingHand/set_flag_on_action", justification: None });
    textures.insert("HDGraphic", TextureDescription { texture: "HDGraphic", justification: Some((0.0, 0.0)) });
    textures.insert("AurorasHelper/PauseMusicWhenPausedController", TextureDescription { texture: "controllers/AurorasHelper/PauseMusicWhenPausedController", justification: None });
    textures.insert("JungleHelper/TreasureChest", TextureDescription { texture: "JungleHelper/Treasure/TreasureIdle00", justification: None });
    textures.insert("SaladimHelper/CollectableCoin", TextureDescription { texture: "SaladimHelper/entities/collectableCoin/idle00", justification: None });
    textures.insert("AuraHelper/Insect", TextureDescription { texture: "objects/insect1", justification: None });
    textures.insert("CherryHelper/ItemCrystal", TextureDescription { texture: "objects/itemCrystal/idle00", justification: Some((0.5, 0.5)) });
    textures.insert("CommunalHelper/SJ/PhotosensitiveFlagController", TextureDescription { texture: "objects/CommunalHelper/strawberryJam/photosensitiveFlagController/icon", justification: None });
    textures.insert("BrokemiaHelper/wallDashSpringLeft", TextureDescription { texture: "objects/BrokemiaHelper/dashSpring/00", justification: Some((0.5, 1.0)) });        
    textures.insert("JungleHelper/Cockatiel", TextureDescription { texture: "JungleHelper/Cockatiel/idle00", justification: None });
    textures.insert("YetAnotherHelper/SpikeJumpThruController", TextureDescription { texture: "ahorn/YetAnotherHelper/spikeJumpThruController", justification: None });   
    textures.insert("CommunalHelper/InputFlagController", TextureDescription { texture: "objects/CommunalHelper/inputFlagController/icon", justification: None });        
    textures.insert("corkr900CoopHelper/SyncedSeeker", TextureDescription { texture: "characters/monsters/predator73", justification: None });
    textures.insert("AuraHelper/Lantern", TextureDescription { texture: "objects/lantern", justification: None });
    textures.insert("AuraHelper/IceSlime", TextureDescription { texture: "objects/iceslime1", justification: None });
    textures.insert("vitellary/starcrystal", TextureDescription { texture: "objects/crystals/star/idle00", justification: None });
    textures.insert("DJMapHelper/playSprite", TextureDescription { texture: "characters/oldlady/idle00", justification: Some((0.5, 1.0)) });
    textures.insert("batteries/battery", TextureDescription { texture: "batteries/battery/full0", justification: Some((0.5, 1.0)) });
    textures.insert("AurorasHelper/SpeedLimitFlagController", TextureDescription { texture: "controllers/AurorasHelper/SpeedLimitFlagController", justification: Some((0.5, 1.0)) });
    textures.insert("DJMapHelper/finalBossReversed", TextureDescription { texture: "characters/badelineBoss/charge00", justification: None });
    textures.insert("CommunalHelper/NoOverlayLookout", TextureDescription { texture: "objects/lookout/lookout05", justification: Some((0.5, 1.0)) });
    textures.insert("FactoryHelper/MachineHeart", TextureDescription { texture: "objects/FactoryHelper/machineHeart/front0", justification: None });
    textures.insert("GlitchHelper/PurpleGlitch", TextureDescription { texture: "objects/glitch/glitchpurple00", justification: None });
    textures.insert("MaxHelpingHand/SetFlagOnFullClearController", TextureDescription { texture: "ahorn/MaxHelpingHand/set_flag_on_spawn", justification: None });        
    textures.insert("VivHelper/GoldenBerryToFlag", TextureDescription { texture: "ahorn/VivHelper/GoldenBerryToFlag", justification: None });
    textures.insert("MaxHelpingHand/ReversibleRetentionBooster", TextureDescription { texture: "objects/MaxHelpingHand/reversibleRetentionBooster/booster00", justification: None });
    textures.insert("pandorasBox/dreamDashController", TextureDescription { texture: "objects/pandorasBox/controllerIcons/dreamDashController", justification: None });   
    textures.insert("BounceHelper/BounceBumper", TextureDescription { texture: "objects/Bumper/Idle22", justification: None });
    textures.insert("DJMapHelper/startPoint", TextureDescription { texture: "characters/player/sitDown15", justification: Some((0.5, 1.0)) });
    textures.insert("GameHelper/Dispenser", TextureDescription { texture: "objects/GameHelper/dispenser", justification: None });
    textures.insert("GlitchHelper/RedGlitch", TextureDescription { texture: "objects/glitch/glitchred00", justification: None });
    textures.insert("ArphimigonHelper/BadelineBoss", TextureDescription { texture: "characters/badelineBoss/charge00", justification: None });
    textures.insert("XaphanHelper/JumpBlocksFlipSoundController", TextureDescription { texture: "@Internal@/sound_source", justification: None });
    textures.insert("CommunalHelper/SyncedZipMoverActivationController", TextureDescription { texture: "objects/CommunalHelper/syncedZipMoverActivationController/syncedZipMoverActivationController", justification: None });
    textures.insert("MaxHelpingHand/FancyTextTutorial", TextureDescription { texture: "ahorn/MaxHelpingHand/greyscale_birb", justification: Some((0.5, 1.0)) });
    textures.insert("MaxHelpingHand/MultiNodeBumper", TextureDescription { texture: "objects/Bumper/Idle22", justification: None });
    textures.insert("XaphanHelper/InGameMapTilesController", TextureDescription { texture: "util/XaphanHelper/Loenn/tilesController", justification: None });
    textures.insert("CommunalHelper/HintController", TextureDescription { texture: "objects/CommunalHelper/hintController/icon", justification: None });
    textures.insert("FactoryHelper/Battery", TextureDescription { texture: "objects/FactoryHelper/batteryBox/battery00", justification: None });
    textures.insert("MaxHelpingHand/DisableControlsController", TextureDescription { texture: "ahorn/MaxHelpingHand/disable_controls", justification: None });
    textures.insert("cavern/fakecavernheart", TextureDescription { texture: "collectables/heartGem/0/00", justification: Some((0.5, 0.5)) });
    textures.insert("CherryHelper/ShadowDashRefill", TextureDescription { texture: "objects/shadowDashRefill/idle00", justification: Some((0.5, 0.5)) });
    textures.insert("CommunalHelper/SeekerDashRefill", TextureDescription { texture: "objects/CommunalHelper/seekerDashRefill/idle00", justification: None });
    textures.insert("TeraHelper/activeTera", TextureDescription { texture: "TeraHelper/objects/tera/Block/Any", justification: None });
    textures.insert("AurorasHelper/WaveCrystal", TextureDescription { texture: "objects/auroras_helper/mode_crystals/wave_crystal/idle00", justification: None });        
    textures.insert("CollabUtils2/SpeedBerry", TextureDescription { texture: "CollabUtils2/speedBerry/Idle_g06", justification: None });
    textures.insert("SSMHelper/ResizableDashSwitch", TextureDescription { texture: "objects/SSMHelper/bigDashSwitch/bigSwitch00", justification: None });
    textures.insert("MaxHelpingHand/FlagBadelineChaser", TextureDescription { texture: "characters/badeline/sleep00", justification: Some((0.5, 1.0)) });
    textures.insert("XaphanHelper/CustomEndScreenController", TextureDescription { texture: "util/XaphanHelper/Loenn/customEndScreenController", justification: None });  
    textures.insert("CherryHelper/FallTeleport", TextureDescription { texture: "objects/temple/portal/portalframe", justification: Some((0.5, 0.5)) });
    textures.insert("pandorasBox/pandorasBox", TextureDescription { texture: "objects/pandorasBox/pandorasBox/box_idle0", justification: Some((0.5, 1.0)) });
    textures.insert("VivHelper/CustomCoreMessage", TextureDescription { texture: "@Internal@/core_message", justification: None });
    textures.insert("JungleHelper/Lantern", TextureDescription { texture: "JungleHelper/Lantern/LanternEntity/lantern_00", justification: None });
    textures.insert("pandorasBox/playerClone", TextureDescription { texture: "characters/player/sitDown00", justification: Some((0.5, 1.0)) });
    textures.insert("XaphanHelper/MergeChaptersController", TextureDescription { texture: "util/XaphanHelper/Loenn/mergeChaptersController", justification: None });      
    textures.insert("MaxHelpingHand/SetFlagOnSpawnController", TextureDescription { texture: "ahorn/MaxHelpingHand/set_flag_on_spawn", justification: None });
    textures.insert("CollabUtils2/CollabCrystalHeart", TextureDescription { texture: "collectables/heartGem/0/00", justification: None });
    textures.insert("BrokemiaHelper/CelesteNetFlagSynchronizer", TextureDescription { texture: "Ahorn/BrokemiaHelper/CelesteNetFlagSynchronizer", justification: None }); 
    textures.insert("CollabUtils2/GymMarker", TextureDescription { texture: "CollabUtils2/editor_gymmarker", justification: None });
    textures.insert("AuraHelper/Slime", TextureDescription { texture: "objects/slime1", justification: None });
    textures.insert("vitellary/flagsequencecontroller", TextureDescription { texture: "ahorn_flagsequencecontroller", justification: None });
    textures.insert("MaxHelpingHand/LitBlueTorch", TextureDescription { texture: "objects/temple/torch03", justification: None });
    textures.insert("corkr900CoopHelper/SyncedKey", TextureDescription { texture: "collectables/key/idle00", justification: None });
    textures.insert("corkr900CoopHelper/SyncedLightningBreakerBox", TextureDescription { texture: "objects/breakerBox/Idle00", justification: None });
    textures.insert("FlaglinesAndSuch/ShyGhost", TextureDescription { texture: "objects/FlaglinesAndSuch/shyghost/chase00", justification: None });
    textures.insert("FlaglinesAndSuch/NailHittableSprite", TextureDescription { texture: "glass", justification: None });
    textures.insert("CherryHelper/ItemCrystalPedestal", TextureDescription { texture: "objects/itemCrystalPedestal/pedestal00", justification: Some((0.5, 0.5)) });       
    textures.insert("XaphanHelper/SetStatsFlagsController", TextureDescription { texture: "util/XaphanHelper/Loenn/setStatsFlagsController ", justification: None });     
    textures.insert("FemtoHelper/VitalDrainController", TextureDescription { texture: "loenn/Femtohelper/vitalcontroller", justification: None });
    textures.insert("BrokemiaHelper/questionableFlagController", TextureDescription { texture: "Ahorn/BrokemiaHelper/questionableFlagController", justification: None }); 
    textures.insert("JungleHelper/Cobweb", TextureDescription { texture: "JungleHelper/Cobweb/idle00", justification: None });
    textures.insert("ParrotHelper/FlagBerryMoon", TextureDescription { texture: "collectables/moonBerry/normal00", justification: None });
    textures.insert("XaphanHelper/UpgradeController", TextureDescription { texture: "util/XaphanHelper/Loenn/upgradeController", justification: None });
    textures.insert("corkr900CoopHelper/SyncedPuffer", TextureDescription { texture: "objects/puffer/idle00", justification: None });
    textures.insert("ChronoHelper/ShatterSpinner", TextureDescription { texture: "danger/crystal/fg00", justification: None });
    textures.insert("CommunalHelper/DreamRefill", TextureDescription { texture: "objects/CommunalHelper/dreamRefill/idle02", justification: None });
    textures.insert("FlaglinesAndSuch/MusicParamOnFlag", TextureDescription { texture: "ahorn/FlaglinesAndSuch/flag_count_music", justification: None });
    textures.insert("FlaglinesAndSuch/Wingmould", TextureDescription { texture: "objects/FlaglinesAndSuch/Wingmould/idle00", justification: None });
    textures.insert("AurorasHelper/TimedFlagController", TextureDescription { texture: "controllers/AurorasHelper/TimedFlagController", justification: Some((0.5, 1.0)) });
    textures.insert("XaphanHelper/CustomBadelineBoss", TextureDescription { texture: "characters/badelineBoss/charge00", justification: None });
    textures.insert("AdventureHelper/StarTrackSpinnerMultinode", TextureDescription { texture: "danger/starfish14", justification: None });
    textures.insert("GameHelper/EntityRespriter", TextureDescription { texture: "loenn/GameHelper/entity_respriter", justification: Some((0.0, 0.0)) });
    textures.insert("MaxHelpingHand/ParallaxFadeOutController", TextureDescription { texture: "@Internal@/northern_lights", justification: None });
    textures.insert("MaxHelpingHand/FlagBreakerBox", TextureDescription { texture: "objects/breakerBox/Idle00", justification: None });
    textures.insert("CommunalHelper/DreamStrawberry", TextureDescription { texture: "collectables/CommunalHelper/dreamberry/wings01", justification: None });
    textures.insert("FactoryHelper/DoorRusty", TextureDescription { texture: "objects/FactoryHelper/doorRusty/metaldoor00", justification: Some((0.5, 1.0)) });
    textures.insert("VivHelper/EarlyFlagSetter", TextureDescription { texture: "ahorn/VivHelper/flagBeforeAwake", justification: None });
    textures.insert("CommunalHelper/CrystalHeart", TextureDescription { texture: "collectables/heartGem/ghost00", justification: None });
    textures.insert("JungleHelper/CheatCodeController", TextureDescription { texture: "ahorn/JungleHelper/cheat_code", justification: None });
    textures.insert("XaphanHelper/InGameMapController", TextureDescription { texture: "util/XaphanHelper/Loenn/mapController", justification: None });
    textures.insert("ArphimigonHelper/CatassaultPhase1", TextureDescription { texture: "objects/catassaultPhaseOne/main13", justification: Some((0.5, 0.5)) });
    textures.insert("MaxHelpingHand/ExpandTriggerController", TextureDescription { texture: "ahorn/MaxHelpingHand/expand_trigger_controller", justification: None });     
    textures.insert("Anonhelper/CloudRefill", TextureDescription { texture: "objects/AnonHelper/cloudRefill/idle00", justification: None });
    textures.insert("ShroomHelper/DoubleRefillBooster", TextureDescription { texture: "objects/sh_doublerefillbooster/boosterPink00", justification: None });
    
    textures
}
