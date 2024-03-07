use std::{collections::HashMap, marker::PhantomData, ops::BitOr};

use anyhow::{anyhow, Context, Result};
use celesteloader::{
    atlas::Sprite,
    map::{Bounds, Decal, Map, Pos, Room},
    tileset::Tileset,
    CelesteInstallation,
};
use tiny_skia::{Color, IntSize, Paint, Pattern, Pixmap, PixmapRef, Rect, Transform};

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

pub trait LookupAsset {
    fn lookup(&self, path: &str) -> Result<Option<Vec<u8>>>;
}

impl<T: LookupAsset> LookupAsset for &T {
    fn lookup(&self, path: &str) -> Result<Option<Vec<u8>>> {
        (**self).lookup(path)
    }
}

pub struct NullLookup;
impl LookupAsset for NullLookup {
    fn lookup(&self, _: &str) -> Result<Option<Vec<u8>>> {
        Ok(None)
    }
}

pub struct CelesteRenderData<L> {
    tileset_fg: HashMap<char, ParsedTileset>,
    tileset_bg: HashMap<char, ParsedTileset>,
    gameplay_atlas: Pixmap,
    gameplay_sprites: HashMap<String, Sprite>,
    scenery: Sprite,

    lookup_asset: L,
    lookup_cache: elsa::FrozenMap<String, Box<Pixmap>>,
}

enum SpriteLocation<'a> {
    Atlas(&'a Sprite),
    Raw(&'a Pixmap),
}

impl<L: LookupAsset> CelesteRenderData<L> {
    fn lookup_gameplay(&self, path: &str) -> Result<SpriteLocation<'_>> {
        if let Some(sprite) = self.gameplay_sprites.get(path.trim_end_matches(".png")) {
            return Ok(SpriteLocation::Atlas(sprite));
        }

        if let Some(cached) = self.lookup_cache.get(path) {
            return Ok(SpriteLocation::Raw(cached));
        }

        if let Some(sprite) = self.lookup_asset.lookup(path)? {
            let pixmap = Pixmap::decode_png(&sprite)
                .with_context(|| anyhow!("failed to decode {} as png", path))?;
            let a = self.lookup_cache.insert(path.to_owned(), Box::new(pixmap));
            return Ok(SpriteLocation::Raw(a));
        }

        Err(anyhow!("could not find '{}'", path))
    }
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
                Some(copy) => built.get(&copy.to_ascii_lowercase()).unwrap().set.clone(),
                _ => Vec::with_capacity(tileset.set.len()),
            };

            for set in &tileset.set {
                let mask = parse_mask_string(&set.mask)
                    .ok_or_else(|| anyhow!("failed to parse tileset mask '{}'", set.mask))?;
                let tiles = parse_set_tiles(&set.tiles)
                    .ok_or_else(|| anyhow!("failed to parse tileset tiles '{}'", set.tiles))?;

                rules.push(MaskData { mask, tiles });
            }
            // TODO sort

            built.insert(
                tileset.id.to_ascii_lowercase(),
                ParsedTileset {
                    path: tileset.path.clone(),
                    set: rules,
                },
            );
        }
        Ok(built)
    }
}

impl CelesteRenderData<NullLookup> {
    pub fn vanilla(celeste: &CelesteInstallation) -> Result<Self> {
        let fgtiles_xml = celeste.read_to_string("Content/Graphics/ForegroundTiles.xml")?;
        let bgtiles_xml = celeste.read_to_string("Content/Graphics/BackgroundTiles.xml")?;

        let mut base = CelesteRenderData::base(celeste, NullLookup)?;
        base.load_tilesets(&fgtiles_xml, &bgtiles_xml)?;

        Ok(base)
    }
}

impl<L> CelesteRenderData<L> {
    pub fn base(celeste: &CelesteInstallation, lookup_asset: L) -> Result<Self> {
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
            tileset_fg: HashMap::new(),
            tileset_bg: HashMap::new(),
            gameplay_atlas,
            scenery,
            gameplay_sprites,
            lookup_asset,
            lookup_cache: Default::default(),
        })
    }

    pub fn load_tilesets(&mut self, fgtiles_xml: &str, bgtiles_xml: &str) -> Result<()> {
        let tileset_fg = celesteloader::tileset::parse_tilesets(&fgtiles_xml)
            .context("error parsing fgtiles")?;
        self.tileset_fg = ParsedTileset::parse(&tileset_fg)?;

        let tileset_bg = celesteloader::tileset::parse_tilesets(&bgtiles_xml)
            .context("error parsing bgtiles")?;
        self.tileset_bg = ParsedTileset::parse(&tileset_bg)?;

        Ok(())
    }
}

pub fn render_with<L: LookupAsset>(
    render_data: &CelesteRenderData<L>,
    map: &Map,
    layer: Layer,
) -> Result<Pixmap> {
    let map_bounds = map.bounds();

    let mut data = Vec::new();
    let size = map_bounds.size.0 as usize * map_bounds.size.1 as usize * 4;
    data.try_reserve(size)?;
    data.resize(data.capacity(), 0);

    let pixmap = Pixmap::from_vec(
        data,
        IntSize::from_wh(map_bounds.size.0, map_bounds.size.1).unwrap(),
    )
    .context("failed to create pixmap")?;

    let mut cx = RenderContext {
        map_bounds,
        pixmap,
        _marker: PhantomData::<L>,
    };
    cx.render_map(&render_data, map, layer, Color::from_rgba8(50, 50, 50, 255))?;

    Ok(cx.pixmap)
}

pub fn render(celeste: &CelesteInstallation, map: &Map, layer: Layer) -> Result<Pixmap> {
    let render_data = CelesteRenderData::vanilla(celeste)?;
    render_with(&render_data, map, layer)
}

struct RenderContext<L> {
    map_bounds: Bounds,
    pixmap: Pixmap,

    _marker: PhantomData<L>,
}

impl<L: LookupAsset> RenderContext<L> {
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

impl<L: LookupAsset> RenderContext<L> {
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
        cx: &CelesteRenderData<L>,
        map_pos: (f32, f32),
        scale: (f32, f32),
        sprite: SpriteLocation,
    ) -> Result<()> {
        let (x, y) = self.transform_pos_f32(map_pos);

        let (real_w, real_h, sprite_w, sprite_h, sprite_offset_x, sprite_offset_y, atlas) =
            match &sprite {
                SpriteLocation::Atlas(sprite) => (
                    sprite.real_w,
                    sprite.real_h,
                    sprite.w,
                    sprite.h,
                    sprite.offset_x,
                    sprite.offset_y,
                    cx.gameplay_atlas.as_ref(),
                ),
                SpriteLocation::Raw(pixmap) => (
                    pixmap.width() as i16,
                    pixmap.height() as i16,
                    pixmap.width() as i16,
                    pixmap.height() as i16,
                    0,
                    0,
                    pixmap.as_ref(),
                ),
            };

        let jx = 0.5;
        let jy = 0.5;

        let draw_x = (x - (real_w as f32 * jx + sprite_offset_x as f32) * scale.0 as f32).floor();
        let draw_y = (y - (real_h as f32 * jy + sprite_offset_y as f32) * scale.1 as f32).floor();

        let pattern_transform = match sprite {
            SpriteLocation::Atlas(sprite) => {
                Transform::from_translate(draw_x - sprite.x as f32, draw_y - sprite.y as f32)
            }
            SpriteLocation::Raw(_) => Transform::from_translate(draw_x, draw_y),
        };

        let scale_transform = Transform::from_translate(-draw_x, -draw_y)
            .post_scale(scale.0, scale.1)
            .post_translate(draw_x, draw_y);

        let rect = Rect::from_xywh(draw_x, draw_y, sprite_w as f32, sprite_h as f32).unwrap();

        self.pixmap.fill_rect(
            rect,
            &Paint {
                shader: Pattern::new(
                    atlas,
                    tiny_skia::SpreadMode::Pad,
                    tiny_skia::FilterQuality::Nearest,
                    1.0,
                    pattern_transform,
                ),
                anti_alias: false,
                ..Default::default()
            },
            scale_transform,
            None,
        );

        Ok(())
    }

    fn tile_sprite(&mut self, atlas: PixmapRef, pos: Pos, atlas_position: (i16, i16)) {
        let (x, y) = self.transform_pos(pos);

        let shader = Pattern::new(
            atlas,
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
        cx: &CelesteRenderData<L>,
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
    fn render_room(&mut self, room: &Room, cx: &CelesteRenderData<L>, layer: Layer) -> Result<()> {
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
        cx: &CelesteRenderData<L>,
    ) -> Result<()> {
        let (w, h) = room.bounds.size_tiles();

        let matrix = tiles_to_matrix(room.bounds.size_tiles(), tiles);

        for x in 0..w {
            for y in 0..h {
                let c = matrix.get(x, y);

                if c == '0' {
                    continue;
                }

                let tileset = tilesets
                    .get(&char::from(c).to_ascii_lowercase())
                    .ok_or_else(|| anyhow!("tileset for '{}' not found", char::from(c)))
                    .context(room.name.clone())?;

                let random_tiles = choose_tile(&tileset, x, y, &matrix)?.unwrap();
                let sprite_tile_offset = fastrand::choice(random_tiles).unwrap();

                let sprite = cx.lookup_gameplay(&format!("tilesets/{}", tileset.path))?;

                let (sprite_x, sprite_y, sprite_offset_x, sprite_offset_y, atlas) = match &sprite {
                    SpriteLocation::Atlas(sprite) => (
                        sprite.x,
                        sprite.y,
                        sprite.offset_x,
                        sprite.offset_y,
                        cx.gameplay_atlas.as_ref(),
                    ),
                    SpriteLocation::Raw(pixmap) => {
                        // dbg!(sprite_tile_offset);
                        (0, 0, 0, 0, pixmap.as_ref())
                    }
                };

                let sprite_pos = (
                    sprite_x + sprite_tile_offset.0 as i16 * 8,
                    sprite_y + sprite_tile_offset.1 as i16 * 8,
                );

                if sprite_offset_x != 0 {
                    panic!();
                }
                if sprite_offset_y != 0 {
                    panic!();
                }

                let tile_pos = room.bounds.position.offset_tile(x as i32, y as i32);
                self.tile_sprite(atlas, tile_pos, sprite_pos);
            }
        }

        Ok(())
    }

    fn render_tileset_scenery(
        &mut self,
        room: &Room,
        tiles: &str,
        cx: &CelesteRenderData<L>,
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
                self.tile_sprite(cx.gameplay_atlas.as_ref(), tile_pos, (sprite_x, sprite_y));
            }
        }

        Ok(())
    }

    fn render_decals(
        &mut self,
        room: &Room,
        decals: &[Decal],
        cx: &CelesteRenderData<L>,
    ) -> Result<()> {
        for decal in decals {
            let map_pos = (
                room.bounds.position.x as f32 + decal.x,
                room.bounds.position.y as f32 + decal.y,
            );

            let sprite = cx.lookup_gameplay(&format!("decals/{}", decal.texture))?;
            self.sprite(cx, map_pos, (decal.scale_x, decal.scale_y), sprite)?;
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

const AIR: char = '0';

fn tiles_to_matrix(tile_size: (u32, u32), tiles: &str) -> Matrix<char> {
    let mut backing = Vec::with_capacity((tile_size.0 * tile_size.1) as usize);

    let mut i = 0;
    for line in tiles.lines() {
        let before = backing.len();
        backing.extend(line.chars());
        let after = backing.len();

        let remaining = tile_size.0 as usize - (after - before);
        backing.resize(backing.len() + remaining, AIR);

        assert_eq!((after - before) + remaining, tile_size.0 as usize);

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
    fn matches(&self, _center: char, neighbor: char) -> bool {
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

    let values: Vec<_> = str.split('-').collect();
    let [a, b, c] = values.as_slice() else {
        // eprintln!("warning: non-3x3 autotiler mask");

        return Some(AutotilerMask::Pattern([
            [
                AutotilerMaskSegment::Wildcard,
                AutotilerMaskSegment::Wildcard,
                AutotilerMaskSegment::Wildcard,
            ],
            [
                AutotilerMaskSegment::Wildcard,
                AutotilerMaskSegment::Wildcard,
                AutotilerMaskSegment::Wildcard,
            ],
            [
                AutotilerMaskSegment::Wildcard,
                AutotilerMaskSegment::Wildcard,
                AutotilerMaskSegment::Wildcard,
            ],
        ]));
    };

    let mask_from_val = |val: u8| match val {
        b'1' => AutotilerMaskSegment::Present,
        b'0' => AutotilerMaskSegment::Absent,
        b'x' => AutotilerMaskSegment::Wildcard,
        _ => unimplemented!("{}", char::from(val)),
    };
    let parse_row = |a: &str| -> [AutotilerMaskSegment; 3] {
        let row = a.bytes().map(mask_from_val).collect::<Vec<_>>();

        if let Ok(val) = row.try_into() {
            val
        } else {
            // eprintln!("warning: non-3x3 autotiler mask");
            [
                AutotilerMaskSegment::Wildcard,
                AutotilerMaskSegment::Wildcard,
                AutotilerMaskSegment::Wildcard,
            ]
        }
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
            let (x, y) = val.trim().split_once(',')?;
            let x = x.parse().ok()?;
            let y = y.parse().ok()?;
            Some((x, y))
        })
        .collect()
}

impl AutotilerMask {
    fn validate(&self, x: u32, y: u32, matrix: &Matrix<char>) -> bool {
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
    tiles: &Matrix<char>,
) -> Result<Option<&'a [(u8, u8)]>> {
    for set in &tileset.set {
        if set.mask.validate(x, y, tiles) {
            return Ok(Some(&set.tiles));
        }
    }

    Ok(None)
}
