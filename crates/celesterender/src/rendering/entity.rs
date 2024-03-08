use std::{borrow::Cow, f32::consts::PI, num::ParseIntError};

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
            let bg = entity.raw.try_get_attr_int("bg")?;

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
        _ => return Ok(false),
    }

    Ok(true)
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
