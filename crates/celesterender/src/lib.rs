use std::{collections::HashMap, ops::BitOr};

use anyhow::{anyhow, Result};
use celesteloader::{
    atlas::Sprite,
    map::{Bounds, Decal, Map, Pos, Room},
    tileset::Tileset,
    CelesteInstallation,
};
use tiny_skia::{Color, IntSize, Paint, Pattern, Pixmap, Rect, Transform};

#[derive(Clone, Copy)]
pub struct Layer(u8);
impl Layer {
    pub const ALL: Layer = Layer(0b00111111);
    pub const TILES_BG: Layer = Layer(1 << 0);
    pub const DECALS_BG: Layer = Layer(1 << 1);
    pub const ENTITIES: Layer = Layer(1 << 2);
    pub const TILES_FG: Layer = Layer(1 << 3);
    pub const DECALS_FG: Layer = Layer(1 << 4);
    pub const TRIGGERS: Layer = Layer(1 << 5);

    pub fn has(self, other: Layer) -> bool {
        self.0 & other.0 == other.0
    }
}
impl BitOr for Layer {
    type Output = Layer;

    fn bitor(self, rhs: Self) -> Self::Output {
        Layer(self.0.bitor(rhs.0))
    }
}

struct CelesteRenderData {
    tileset_fg: HashMap<char, ParsedTileset>,
    tileset_bg: HashMap<char, ParsedTileset>,
    gameplay_atlas: Pixmap,
    gameplay_sprites: HashMap<String, Sprite>,
    scenery: Sprite,
}

#[derive(Clone)]
struct ParsedTileset {
    path: String,
    set: Vec<MaskData>,
}

impl ParsedTileset {
    pub fn parse(tilesets: &[Tileset]) -> Result<HashMap<char, ParsedTileset>> {
        let mut built = HashMap::<char, ParsedTileset>::with_capacity(tilesets.len());
        for tileset in tilesets {
            let mut rules = match tileset.copy {
                Some(copy) => built.get(&copy).unwrap().set.clone(),
                _ => Vec::with_capacity(tileset.set.len()),
            };

            for set in &tileset.set {
                let mask = parse_mask_string(&set.mask)
                    .ok_or_else(|| anyhow!("failed to parse tileset mask"))?;
                let tiles = parse_set_tiles(&set.tiles)
                    .ok_or_else(|| anyhow!("failed to parse tileset tiles"))?;

                rules.push(MaskData { mask, tiles });
            }
            // TODO sort

            built.insert(
                tileset.id,
                ParsedTileset {
                    path: tileset.path.clone(),
                    set: rules,
                },
            );
        }
        Ok(built)
    }
}

impl CelesteRenderData {
    pub fn new(celeste: &CelesteInstallation) -> Result<Self> {
        let fgtiles_xml = celeste.read_to_string("Content/Graphics/ForegroundTiles.xml")?;
        let tileset_fg = celesteloader::tileset::parse_tilesets(&fgtiles_xml)?;
        let tileset_fg = ParsedTileset::parse(&tileset_fg)?;

        let bgtiles_xml = celeste.read_to_string("Content/Graphics/BackgroundTiles.xml")?;
        let tileset_bg = celesteloader::tileset::parse_tilesets(&bgtiles_xml)?;
        let tileset_bg = ParsedTileset::parse(&tileset_bg)?;

        let gameplay_atlas_meta = celeste.gameplay_atlas()?;
        let gameplay_atlas_image = celeste.decode_atlas_image(&gameplay_atlas_meta)?;
        let gameplay_atlas = Pixmap::from_vec(
            gameplay_atlas_image.2,
            IntSize::from_wh(gameplay_atlas_image.0, gameplay_atlas_image.1).expect("atlas size"),
        )
        .expect("atlas size");

        let scenery = gameplay_atlas_meta
            .sprites
            .iter()
            .find(|i| i.path == "tilesets/scenery")
            .expect("no scenery sprite")
            .clone();

        let gameplay_sprites = gameplay_atlas_meta
            .sprites
            .into_iter()
            .map(|sprite| (sprite.path.clone(), sprite))
            .collect::<HashMap<_, _>>();

        Ok(CelesteRenderData {
            tileset_fg,
            tileset_bg,
            gameplay_atlas,
            scenery,
            gameplay_sprites,
        })
    }
}

pub fn render(celeste: &CelesteInstallation, map: &Map, layer: Layer) -> Result<Pixmap> {
    let render_data = CelesteRenderData::new(celeste)?;

    let map_bounds = map.bounds();
    let pixmap =
        tiny_skia::Pixmap::new(map_bounds.size.0, map_bounds.size.1).expect("nonzero map size");

    let mut cx = RenderContext { map_bounds, pixmap };
    cx.render_map(&render_data, map, layer, Color::from_rgba8(50, 50, 50, 255))?;

    Ok(cx.pixmap)
}

struct RenderContext {
    map_bounds: Bounds,
    pixmap: Pixmap,
}

impl RenderContext {
    /// World space to image space
    fn transform_pos(&self, pos: Pos) -> (i32, i32) {
        let top_left = self.map_bounds.position;
        (pos.x - top_left.x, pos.y - top_left.y)
    }
    fn transform_pos_f32(&self, (x, y): (f32, f32)) -> (f32, f32) {
        let top_left = self.map_bounds.position;
        (x - top_left.x as f32, y - top_left.y as f32)
    }

    /// World space to image space
    fn transform_bounds(&self, bounds: Bounds) -> Rect {
        let pos = self.transform_pos(bounds.position);
        Rect::from_xywh(
            pos.0 as f32,
            pos.1 as f32,
            bounds.size.0 as f32,
            bounds.size.1 as f32,
        )
        .unwrap()
    }
}

impl RenderContext {
    fn _rect(&mut self, rect: Rect, color: Color) {
        self.pixmap.fill_rect(
            rect,
            &Paint {
                shader: tiny_skia::Shader::SolidColor(color),
                ..Default::default()
            },
            Transform::identity(),
            None,
        );
    }

    fn sprite(
        &mut self,
        cx: &CelesteRenderData,
        map_pos: (f32, f32),
        scale: (i32, i32),
        sprite: &Sprite,
    ) {
        let (x, y) = self.transform_pos_f32(map_pos);

        // assert_eq!(sprite.offset_x, 0);
        // assert_eq!(sprite.offset_y, 0);

        let shader = Pattern::new(
            cx.gameplay_atlas.as_ref(),
            tiny_skia::SpreadMode::Repeat,
            tiny_skia::FilterQuality::Nearest,
            1.0,
            Transform::from_translate(-sprite.x as f32, -sprite.y as f32),
        );
        let jx = 0.5;
        let jy = 0.5;

        let mut draw_x =
            (x - (sprite.real_w as f32 * jx + sprite.offset_x as f32) * scale.0 as f32).floor();
        let mut draw_y =
            (y - (sprite.real_h as f32 * jy + sprite.offset_y as f32) * scale.1 as f32).floor();

        if scale.0 < 0 {
            draw_x += sprite.w as f32 * scale.0 as f32;
        }
        if scale.1 < 0 {
            draw_y += sprite.h as f32 * scale.1 as f32;
        }

        if scale.0 != 1 && scale.0 != -1 {
            panic!("{}", scale.0);
        }
        if scale.1 != 1 && scale.1 != -1 {
            panic!("{}", scale.1);
        }

        let draw_w = sprite.w as i32 * scale.0.abs();
        let draw_h = sprite.h as i32 * scale.1.abs();

        // TODO rotation

        self.pixmap.fill_rect(
            Rect::from_xywh(0.0, 0.0, draw_w as f32, draw_h as f32).unwrap(),
            &Paint {
                shader,
                ..Default::default()
            },
            Transform::from_translate(draw_x, draw_y),
            None,
        );
    }

    fn tile_sprite(&mut self, cx: &CelesteRenderData, pos: Pos, atlas_position: (i16, i16)) {
        let (x, y) = self.transform_pos(pos);

        let shader = Pattern::new(
            cx.gameplay_atlas.as_ref(),
            tiny_skia::SpreadMode::Repeat,
            tiny_skia::FilterQuality::Nearest,
            1.0,
            Transform::from_translate(-atlas_position.0 as f32, -atlas_position.1 as f32),
        );

        self.pixmap.fill_rect(
            Rect::from_xywh(0.0, 0.0, 8.0, 8.0).unwrap(),
            &Paint {
                shader,
                ..Default::default()
            },
            Transform::from_translate(x as f32, y as f32),
            None,
        );
    }

    fn render_map(
        &mut self,
        cx: &CelesteRenderData,
        map: &Map,
        layer: Layer,
        background: Color,
    ) -> Result<()> {
        self.pixmap.fill(background);

        for room in &map.rooms {
            self.render_room(room, cx, layer)?;
        }

        Ok(())
    }
    fn render_room(&mut self, room: &Room, cx: &CelesteRenderData, layer: Layer) -> Result<()> {
        if false {
            let mut pb = tiny_skia::PathBuilder::new();
            pb.push_rect(self.transform_bounds(room.bounds));
            let path = pb.finish().unwrap();
            self.pixmap.stroke_path(
                &path,
                &Paint::default(),
                &tiny_skia::Stroke::default(),
                Transform::identity(),
                None,
            );
        }

        if layer.has(Layer::TILES_BG) {
            self.render_tileset(room, &room.bg_tiles_raw, &cx.tileset_bg, cx)?;
            self.render_tileset_scenery(room, &room.scenery_bg_raw, cx)?;
        }
        if layer.has(Layer::DECALS_BG) {
            self.render_decals(room, &room.decals_bg, cx)?;
        }
        if layer.has(Layer::ENTITIES) {
            // entity
        }
        if layer.has(Layer::TILES_FG) {
            self.render_tileset(room, &room.fg_tiles_raw, &cx.tileset_fg, cx)?;
            self.render_tileset_scenery(room, &room.scenery_fg_raw, cx)?;
        }
        if layer.has(Layer::DECALS_FG) {
            self.render_decals(room, &room.decals_fg, cx)?;
        }
        if layer.has(Layer::TRIGGERS) {
            // trigger
        }

        Ok(())
    }

    fn render_tileset(
        &mut self,
        room: &Room,
        tiles: &str,
        tilesets: &HashMap<char, ParsedTileset>,
        cx: &CelesteRenderData,
    ) -> Result<()> {
        let (w, h) = room.bounds.size_tiles();

        let matrix = tiles_to_matrix(room.bounds.size_tiles(), tiles);

        for x in 0..w {
            for y in 0..h {
                let c = matrix.get(x, y);

                if c == b'0' {
                    continue;
                }

                let tileset = tilesets
                    .get(&char::from(c))
                    .ok_or_else(|| anyhow!("tileset for '{}' not found", char::from(c)))?;

                let random_tiles = choose_tile(&tileset, x, y, &matrix)?.unwrap();
                let sprite_tile_offset = fastrand::choice(random_tiles).unwrap();

                let sprite = cx
                    .gameplay_sprites
                    .get(&format!("tilesets/{}", tileset.path))
                    .ok_or_else(|| anyhow!("could not find sprite matching '{}'", tileset.path))?;

                let sprite_pos = (
                    sprite.x + sprite_tile_offset.0 as i16 * 8,
                    sprite.y + sprite_tile_offset.1 as i16 * 8,
                );

                if sprite.offset_x != 0 {
                    panic!();
                }
                if sprite.offset_y != 0 {
                    panic!();
                }

                let tile_pos = room.bounds.position.offset_tile(x as i32, y as i32);
                self.tile_sprite(cx, tile_pos, sprite_pos);
            }
        }

        Ok(())
    }

    fn render_tileset_scenery(
        &mut self,
        room: &Room,
        tiles: &str,
        cx: &CelesteRenderData,
    ) -> Result<()> {
        let (w, h) = room.bounds.size_tiles();

        let matrix = tiles_to_matrix_scenery(room.bounds.size_tiles(), tiles);

        for x in 0..w {
            for y in 0..h {
                let index = matrix.get(x, y);

                if index == -1 {
                    continue;
                }

                let scenery_width = cx.scenery.real_w / 8;
                let _scenery_height = cx.scenery.real_h / 8;
                let quad_x = index % scenery_width;
                let quad_y = index / scenery_width;

                let sprite_x = cx.scenery.x - cx.scenery.offset_x + quad_x * 8;
                let sprite_y = cx.scenery.y - cx.scenery.offset_y + quad_y * 8;
                let _w = 8;
                let _h = 8;

                let tile_pos = room.bounds.position.offset_tile(x as i32, y as i32);
                self.tile_sprite(cx, tile_pos, (sprite_x, sprite_y));
            }
        }

        Ok(())
    }

    fn render_decals(
        &mut self,
        room: &Room,
        decals: &[Decal],
        cx: &CelesteRenderData,
    ) -> Result<()> {
        for decal in decals {
            let tex = &format!("decals/{}", decal.texture.trim_end_matches(".png"));
            let sprite = cx
                .gameplay_sprites
                .get(tex)
                .ok_or_else(|| anyhow!("could not find decal for '{}'", decal.texture))?;

            let map_pos = (
                room.bounds.position.x as f32 + decal.x,
                room.bounds.position.y as f32 + decal.y,
            );
            self.sprite(cx, map_pos, (decal.scale_x, decal.scale_y), sprite);
        }

        Ok(())
    }
}

fn tiles_to_matrix_scenery(tile_size: (u32, u32), tiles: &str) -> Matrix<i16> {
    let mut backing = Vec::with_capacity((tile_size.0 * tile_size.1) as usize);

    let mut i = 0;
    for line in tiles.lines() {
        let before = backing.len();
        if !line.is_empty() {
            backing.extend(line.split(',').map(|val| val.parse::<i16>().unwrap()));
        }
        let after = backing.len();

        let remaining = tile_size.0 as usize - (after - before);
        for _ in 0..remaining {
            backing.push(-1);
        }

        assert_eq!((after - before) + remaining, tile_size.0 as usize);

        i += 1;
    }
    let remaining_lines = tile_size.1 as usize - i;

    for _ in 0..remaining_lines {
        for _ in 0..tile_size.0 {
            backing.push(-1);
        }
    }

    assert_eq!(backing.len(), (tile_size.0 * tile_size.1) as usize);

    Matrix {
        size: tile_size,
        backing,
    }
}

const AIR: u8 = b'0';

fn tiles_to_matrix(tile_size: (u32, u32), tiles: &str) -> Matrix<u8> {
    let mut backing = Vec::with_capacity((tile_size.0 * tile_size.1) as usize);

    let mut i = 0;
    for line in tiles.lines() {
        backing.extend(line.bytes());

        let remaining = tile_size.0 as usize - line.len();
        backing.resize(backing.len() + remaining, AIR);

        assert_eq!(line.len() + remaining, tile_size.0 as usize);

        i += 1;
    }
    let remaining_lines = tile_size.1 as usize - i;
    backing.resize(backing.len() + tile_size.0 as usize * remaining_lines, AIR);

    assert_eq!(backing.len(), (tile_size.0 * tile_size.1) as usize);

    Matrix {
        size: tile_size,
        backing,
    }
}

struct Matrix<T> {
    size: (u32, u32),
    backing: Vec<T>,
}

impl<T: Copy> Matrix<T> {
    fn get(&self, x: u32, y: u32) -> T {
        assert!(x < self.size.0);
        let idx = self.size.0 * y + x;
        self.backing[idx as usize]
    }
    fn get_or(&self, x: i32, y: i32, default: T) -> T {
        if x >= self.size.0 as i32 || x < 0 {
            return default;
        }
        if y >= self.size.1 as i32 || y < 0 {
            return default;
        }

        let idx = self.size.0 * y as u32 + x as u32;
        self.backing.get(idx as usize).copied().unwrap_or(default)
    }
}

fn split_twice(s: &str, delim: char) -> Option<(&str, &str, &str)> {
    let (a, rest) = s.split_once(delim)?;
    let (b, c) = rest.split_once(delim)?;

    Some((a, b, c))
}

#[derive(Clone)]
struct MaskData {
    mask: AutotilerMask,
    tiles: Vec<(u8, u8)>,
}

#[derive(Debug, Clone)]
enum AutotilerMask {
    Padding,
    Center,
    Pattern([[AutotilerMaskSegment; 3]; 3]),
}

#[derive(Debug, Clone, Copy)]
enum AutotilerMaskSegment {
    Present,
    Absent,
    Wildcard,
}
impl AutotilerMaskSegment {
    fn matches(&self, _center: u8, neighbor: u8) -> bool {
        match self {
            AutotilerMaskSegment::Present => neighbor != AIR,
            AutotilerMaskSegment::Absent => neighbor == AIR,
            AutotilerMaskSegment::Wildcard => true,
        }
    }
}
fn parse_mask_string(str: &str) -> Option<AutotilerMask> {
    match str {
        "padding" => return Some(AutotilerMask::Padding),
        "center" => return Some(AutotilerMask::Center),
        _ => {}
    }

    let (a, b, c) = split_twice(str, '-')?;

    let mask_from_val = |val: u8| match val {
        b'1' => AutotilerMaskSegment::Present,
        b'0' => AutotilerMaskSegment::Absent,
        b'x' => AutotilerMaskSegment::Wildcard,
        _ => unimplemented!("{}", char::from(val)),
    };
    let parse_row = |a: &str| -> [AutotilerMaskSegment; 3] {
        return a
            .bytes()
            .map(mask_from_val)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
    };

    Some(AutotilerMask::Pattern([
        parse_row(a),
        parse_row(b),
        parse_row(c),
    ]))
}

fn parse_set_tiles(str: &str) -> Option<Vec<(u8, u8)>> {
    str.split(';')
        .map(|val| {
            let (x, y) = val.split_once(',')?;
            let x = x.parse().ok()?;
            let y = y.parse().ok()?;
            Some((x, y))
        })
        .collect()
}

impl AutotilerMask {
    fn validate(&self, x: u32, y: u32, matrix: &Matrix<u8>) -> bool {
        let center = matrix.get(x, y);
        match self {
            AutotilerMask::Padding => {
                let left = matrix.get_or(x as i32 - 2, y as i32, center);
                let right = matrix.get_or(x as i32 + 2, y as i32, center);
                let up = matrix.get_or(x as i32, y as i32 - 2, center);
                let down = matrix.get_or(x as i32, y as i32 + 2, center);

                // TODO ignores
                left == AIR || right == AIR || up == AIR || down == AIR
            }
            AutotilerMask::Center => true,
            #[rustfmt::skip]
            #[allow(clippy::identity_op)]
            AutotilerMask::Pattern(pattern) => {
                       pattern[0][0].matches(center, matrix.get_or(x as i32  - 1, y as i32 - 1, center))
                    && pattern[0][1].matches(center, matrix.get_or(x as i32  + 0, y as i32 - 1, center))
                    && pattern[0][2].matches(center, matrix.get_or(x as i32  + 1, y as i32 - 1, center))
                    && pattern[1][0].matches(center, matrix.get_or(x as i32  - 1, y as i32 + 0, center))
                    && pattern[1][1].matches(center, matrix.get_or(x as i32  + 0, y as i32 + 0, center))
                    && pattern[1][2].matches(center, matrix.get_or(x as i32  + 1, y as i32 + 0, center))
                    && pattern[2][0].matches(center, matrix.get_or(x as i32  - 1, y as i32 + 1, center))
                    && pattern[2][1].matches(center, matrix.get_or(x as i32  + 0, y as i32 + 1, center))
                    && pattern[2][2].matches(center, matrix.get_or(x as i32  + 1, y as i32 + 1, center))
            },
        }
    }
}

fn choose_tile<'a>(
    tileset: &'a ParsedTileset,
    x: u32,
    y: u32,
    tiles: &Matrix<u8>,
) -> Result<Option<&'a [(u8, u8)]>> {
    for set in &tileset.set {
        if set.mask.validate(x, y, tiles) {
            return Ok(Some(&set.tiles));
        }
    }

    Ok(None)
}
