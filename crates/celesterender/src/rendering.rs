pub mod entity;

use std::{
    collections::{BTreeMap, HashMap},
    marker::PhantomData,
    ops::BitOr,
};

use anyhow::{anyhow, ensure, Context, Result};
use celesteloader::{
    atlas::Sprite,
    map::{Bounds, Decal, Map, Pos, Room},
    tileset::Tileset,
    CelesteInstallation,
};
use tiny_skia::{
    Color, IntSize, Paint, PathBuilder, Pattern, Pixmap, PixmapRef, Rect, Shader, Stroke, Transform,
};

use crate::asset::{AssetDb, LookupAsset, NullLookup};

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

pub struct CelesteRenderData {
    pub gameplay_sprites: HashMap<String, Sprite>,
    tileset_fg: HashMap<char, ParsedTileset>,
    tileset_bg: HashMap<char, ParsedTileset>,
    gameplay_atlas: Pixmap,
    scenery: Sprite,
}

#[derive(Debug)]
pub enum SpriteLocation<'a> {
    Atlas(&'a Sprite),
    Raw(&'a Pixmap),
}
impl SpriteLocation<'_> {
    pub fn width(&self) -> i16 {
        match self {
            SpriteLocation::Atlas(sprite) => sprite.real_w,
            SpriteLocation::Raw(pixmap) => pixmap.width() as i16,
        }
    }
    pub fn height(&self) -> i16 {
        match self {
            SpriteLocation::Atlas(sprite) => sprite.real_h,
            SpriteLocation::Raw(pixmap) => pixmap.height() as i16,
        }
    }

    pub fn as_sprite(&self) -> Option<&Sprite> {
        match self {
            SpriteLocation::Atlas(sprite) => Some(sprite),
            SpriteLocation::Raw(_) => None,
        }
    }
}

impl<L: LookupAsset> AssetDb<L> {
    /*pub fn lookup_exact<'a>(
        &'a mut self,
        path: &str,
    ) -> Result<Option<(Vec<u8>, Option<&mut ModArchive>)>> {
        self.lookup_asset.lookup_exact(path)
    }*/

    pub fn lookup_gameplay<'a>(
        &'a mut self,
        cx: &'a CelesteRenderData,
        path: &str,
    ) -> Result<SpriteLocation<'a>> {
        if let Some(sprite) = cx.gameplay_sprites.get(path.trim_end_matches(".png")) {
            return Ok(SpriteLocation::Atlas(sprite));
        }

        if let Some(cached) = self.lookup_cache.get(path) {
            return Ok(SpriteLocation::Raw(cached));
        }

        if let Some(sprite) = self.lookup_asset.lookup_gameplay_png(path)? {
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
    ignores: Ignores,
    set: Vec<MaskData>,
}

#[derive(Clone, Debug)]
pub enum Ignores {
    All,
    None,
    List(Vec<char>),
}
impl Ignores {
    pub fn ignores(&self, c: char) -> bool {
        match self {
            Ignores::All => true,
            Ignores::None => false,
            Ignores::List(list) => list.contains(&c),
        }
    }
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
                    .ok_or_else(|| anyhow!("failed to parse tileset mask '{}'", set.mask))?;
                let tiles = parse_set_tiles(&set.tiles)
                    .ok_or_else(|| anyhow!("failed to parse tileset tiles '{}'", set.tiles))?;

                rules.push(MaskData { mask, tiles });
            }

            let ignores = tileset
                .ignores
                .as_ref()
                .map(|ignores| -> Result<_> {
                    let ignores = ignores.trim();

                    if ignores.is_empty() {
                        Ok(Ignores::None)
                    } else if ignores == "*" {
                        Ok(Ignores::All)
                    } else {
                        let list = ignores
                            .split(',')
                            .map(|x| {
                                if x.chars().count() != 1 {
                                    return None;
                                }

                                Some(x.chars().next().unwrap())
                            })
                            .collect::<Option<Vec<_>>>()
                            .ok_or_else(|| anyhow!("failed to parse ignores '{ignores}'"))?;

                        Ok(Ignores::List(list))
                    }
                })
                .transpose()?
                .unwrap_or(Ignores::None);

            // TODO sort

            built.insert(
                tileset.id,
                ParsedTileset {
                    path: tileset.path.clone(),
                    ignores: ignores,
                    set: rules,
                },
            );
        }
        Ok(built)
    }
}

impl CelesteRenderData {
    pub fn vanilla(celeste: &CelesteInstallation) -> Result<Self> {
        let fgtiles_xml = celeste.read_to_string("Content/Graphics/ForegroundTiles.xml")?;
        let bgtiles_xml = celeste.read_to_string("Content/Graphics/BackgroundTiles.xml")?;

        let mut base = CelesteRenderData::base(celeste)?;
        base.load_tilesets(&fgtiles_xml, &bgtiles_xml)?;

        Ok(base)
    }
}

impl CelesteRenderData {
    pub fn base(celeste: &CelesteInstallation) -> Result<Self> {
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
            gameplay_sprites,
            scenery,
        })
    }

    pub fn load_tilesets(&mut self, fgtiles_xml: &str, bgtiles_xml: &str) -> Result<()> {
        let tileset_fg =
            celesteloader::tileset::parse_tilesets(fgtiles_xml).context("error parsing fgtiles")?;
        self.tileset_fg = ParsedTileset::parse(&tileset_fg)?;

        let tileset_bg =
            celesteloader::tileset::parse_tilesets(bgtiles_xml).context("error parsing bgtiles")?;
        self.tileset_bg = ParsedTileset::parse(&tileset_bg)?;

        Ok(())
    }
}

pub struct RenderResult {
    pub image: Pixmap,
    pub bounds: Bounds,
    pub unknown_entities: BTreeMap<String, u32>,
}

pub struct RenderMapSettings<'a> {
    pub layer: Layer,
    pub include_room: &'a dyn Fn(&Room) -> bool,
}
impl<'a> Default for RenderMapSettings<'a> {
    fn default() -> Self {
        Self {
            layer: Layer::ALL,
            include_room: &|_| true,
        }
    }
}

pub fn render_with<L: LookupAsset>(
    render_data: &CelesteRenderData,
    asset_db: &mut AssetDb<L>,
    map: &Map,
    settings: RenderMapSettings,
) -> Result<RenderResult> {
    fastrand::seed(2);

    let mut map_bounds = Bounds::empty();
    let mut rooms = Vec::new();
    for room in &map.rooms {
        if (settings.include_room)(room) {
            map_bounds = map_bounds.join(room.bounds);
            rooms.push(room);
        }
    }

    ensure!(!rooms.is_empty(), "No rooms to render");

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
        unknown_entities: Default::default(),
        _marker: PhantomData::<L>,
    };

    cx.pixmap.fill(Color::from_rgba8(50, 50, 50, 255));
    for room in rooms {
        cx.render_room(room, render_data, asset_db, settings.layer)?;
    }

    Ok(RenderResult {
        image: cx.pixmap,
        bounds: map_bounds,
        unknown_entities: cx.unknown_entities,
    })
}

pub fn render(
    celeste: &CelesteInstallation,
    map: &Map,
    settings: RenderMapSettings<'_>,
) -> Result<RenderResult> {
    let render_data = CelesteRenderData::vanilla(celeste)?;

    render_with(
        &render_data,
        &mut AssetDb {
            lookup_asset: NullLookup,
            lookup_cache: Default::default(),
        },
        map,
        settings,
    )
}

pub(crate) struct RenderContext<L> {
    map_bounds: Bounds,
    pixmap: Pixmap,
    unknown_entities: BTreeMap<String, u32>,
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
    pub fn circle(&mut self, pos: (f32, f32), radius: f32, color: Color) {
        let (x, y) = self.transform_pos_f32(pos);

        let mut pb = PathBuilder::new();
        pb.push_circle(x, y, radius);

        self.pixmap.stroke_path(
            &pb.finish().unwrap(),
            &Paint {
                shader: tiny_skia::Shader::SolidColor(color),
                anti_alias: false,
                blend_mode: tiny_skia::BlendMode::Plus,

                ..Default::default()
            },
            &Stroke::default(),
            Transform::identity(),
            None,
        );
    }
    fn rect(&mut self, rect: Rect, color: Color) {
        self.pixmap.fill_rect(
            rect,
            &Paint {
                shader: tiny_skia::Shader::SolidColor(color),
                anti_alias: false,
                blend_mode: tiny_skia::BlendMode::Plus,

                ..Default::default()
            },
            Transform::identity(),
            None,
        );
    }
    fn stroke_rect(&mut self, rect: Rect, color: Color) {
        let rect = Rect::from_ltrb(
            rect.left(),
            rect.top(),
            rect.right() - 1.0,
            rect.bottom() - 1.0,
        )
        .unwrap();

        let mut pb = PathBuilder::new();
        pb.push_rect(rect);

        self.pixmap.stroke_path(
            &pb.finish().unwrap(),
            &Paint {
                shader: tiny_skia::Shader::SolidColor(color),
                anti_alias: false,
                ..Default::default()
            },
            &Stroke::default(),
            Transform::identity(),
            None,
        );
    }

    pub(crate) fn sprite(
        &mut self,
        cx: &CelesteRenderData,
        map_pos: (f32, f32),
        scale: (f32, f32),
        justify: (f32, f32),
        sprite: SpriteLocation,
        quad: Option<(i16, i16, i16, i16)>,
        tint: Option<Color>,
    ) -> Result<()> {
        // TODO: tint should only tint the sprite itself

        let (x, y) = self.transform_pos_f32(map_pos);

        let (
            mut real_w,
            mut real_h,
            mut sprite_w,
            mut sprite_h,
            sprite_offset_x,
            sprite_offset_y,
            atlas,
        ) = match &sprite {
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

        let (quad_x, quad_y) = if let Some((quad_x, quad_y, quad_w, quad_h)) = quad {
            real_w = quad_w;
            sprite_w = quad_w;
            real_h = quad_h;
            sprite_h = quad_h;

            (quad_x, quad_y)
        } else {
            (0, 0)
        };

        let draw_x = (x - (real_w as f32 * justify.0 + sprite_offset_x as f32) * scale.0).floor();
        let draw_y = (y - (real_h as f32 * justify.1 + sprite_offset_y as f32) * scale.1).floor();

        let pattern_transform = match sprite {
            SpriteLocation::Atlas(sprite) => Transform::from_translate(
                draw_x - sprite.x as f32 - quad_x as f32,
                draw_y - sprite.y as f32 - quad_y as f32,
            ),
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

        if let Some(tint) = tint {
            let mask = match sprite {
                SpriteLocation::Atlas(_) => None,
                SpriteLocation::Raw(_) => {
                    todo!()
                    // Some(Mask::from_pixmap(raw.as_ref(), tiny_skia::MaskType::Alpha))
                }
            };

            self.pixmap.fill_rect(
                rect,
                &Paint {
                    shader: Shader::SolidColor(tint),
                    blend_mode: tiny_skia::BlendMode::Multiply,
                    anti_alias: false,
                    ..Default::default()
                },
                scale_transform,
                mask.as_ref(),
            );
        }

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

    fn render_room(
        &mut self,
        room: &Room,
        cx: &CelesteRenderData,
        asset_db: &mut AssetDb<L>,
        layer: Layer,
    ) -> Result<()> {
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

        let bgtiles = tiles_to_matrix(room.bounds.size_tiles(), &room.bg_tiles_raw)?;
        let fgtiles = tiles_to_matrix(room.bounds.size_tiles(), &room.fg_tiles_raw)?;

        if layer.has(Layer::TILES_BG) {
            self.render_tileset(room, &bgtiles, &cx.tileset_bg, cx, asset_db)?;
            self.render_tileset_scenery(room, &room.scenery_bg_raw, cx)?;
        }
        if layer.has(Layer::DECALS_BG) {
            self.render_decals(room, &room.decals_bg, cx, asset_db)?;
        }
        if layer.has(Layer::ENTITIES) {
            // TODO: sort by depth
            self.render_entities(room, &fgtiles, cx, asset_db)?;
        }
        if layer.has(Layer::TILES_FG) {
            self.render_tileset(room, &fgtiles, &cx.tileset_fg, cx, asset_db)?;
            self.render_tileset_scenery(room, &room.scenery_fg_raw, cx)?;
        }
        if layer.has(Layer::DECALS_FG) {
            self.render_decals(room, &room.decals_fg, cx, asset_db)?;
        }
        if layer.has(Layer::TRIGGERS) {
            // trigger
        }

        Ok(())
    }

    fn render_tileset(
        &mut self,
        room: &Room,
        tiles: &Matrix<char>,
        tilesets: &HashMap<char, ParsedTileset>,
        cx: &CelesteRenderData,
        asset_db: &mut AssetDb<L>,
    ) -> Result<()> {
        let (w, h) = room.bounds.size_tiles();

        for x in 0..w {
            for y in 0..h {
                let c = tiles.get(x, y);

                if c == '0' {
                    continue;
                }

                let tileset = tilesets
                    .get(&c)
                    .ok_or_else(|| anyhow!("tileset for '{}' not found", c))
                    .context(room.name.clone())?;

                let random_tiles = choose_tile(tileset, x, y, &tiles)?.unwrap();
                let sprite_tile_offset = fastrand::choice(random_tiles).unwrap();

                let sprite = asset_db.lookup_gameplay(cx, &format!("tilesets/{}", tileset.path))?;

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
                self.tile_sprite(cx.gameplay_atlas.as_ref(), tile_pos, (sprite_x, sprite_y));
            }
        }

        Ok(())
    }

    fn render_decals(
        &mut self,
        room: &Room,
        decals: &[Decal],
        cx: &CelesteRenderData,
        asset_db: &mut AssetDb<L>,
    ) -> Result<()> {
        for decal in decals {
            let map_pos = (
                room.bounds.position.x as f32 + decal.x,
                room.bounds.position.y as f32 + decal.y,
            );

            let sprite = asset_db.lookup_gameplay(cx, &format!("decals/{}", decal.texture))?;
            self.sprite(
                cx,
                map_pos,
                (decal.scale_x, decal.scale_y),
                (0.5, 0.5),
                sprite,
                None,
                None,
            )?;
        }

        Ok(())
    }

    fn render_entities(
        &mut self,
        room: &Room,
        fgtiles: &Matrix<char>,
        cx: &CelesteRenderData,
        asset_db: &mut AssetDb<L>,
    ) -> Result<()> {
        for e in &room.entities {
            entity::pre_render_entity(self, cx, asset_db, room, e)?;
        }
        for e in &room.entities {
            if !entity::render_entity(self, fgtiles, cx, asset_db, room, e)? {
                *self.unknown_entities.entry(e.name.clone()).or_default() += 1;
            }
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

pub(crate) const AIR: char = '0';

fn tiles_to_matrix(tile_size: (u32, u32), tiles: &str) -> Result<Matrix<char>> {
    let mut backing = Vec::with_capacity((tile_size.0 * tile_size.1) as usize);

    let mut i = 0;
    for line in tiles.lines() {
        let before = backing.len();
        backing.extend(line.chars());
        let after = backing.len();
        let added = after - before;

        let remaining = (tile_size.0 as isize) - added as isize; // lvl_resort-credits says hello
        backing.resize((backing.len() as isize + remaining) as usize, AIR);

        assert_eq!(added as isize + remaining, tile_size.0 as isize);

        i += 1;
    }
    let remaining_lines = tile_size.1 as usize - i;
    backing.resize(backing.len() + tile_size.0 as usize * remaining_lines, AIR);

    assert_eq!(backing.len(), (tile_size.0 * tile_size.1) as usize);

    Ok(Matrix {
        size: tile_size,
        backing,
    })
}

pub(crate) struct Matrix<T> {
    size: (u32, u32),
    backing: Vec<T>,
}

impl<T: Copy> Matrix<T> {
    pub(crate) fn get(&self, x: u32, y: u32) -> T {
        assert!(x < self.size.0);
        let idx = self.size.0 * y + x;
        self.backing[idx as usize]
    }
    pub(crate) fn get_or(&self, x: i32, y: i32, default: T) -> T {
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
    fn matches(&self, center: char, neighbor: char, ignores: &Ignores) -> bool {
        match self {
            AutotilerMaskSegment::Present => neighbor != AIR,
            // AutotilerMaskSegment::Present => neighbor != AIR,
            AutotilerMaskSegment::Absent => {
                neighbor == AIR || (neighbor != center && ignores.ignores(neighbor))
            }
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
    fn validate(&self, x: u32, y: u32, matrix: &Matrix<char>, ignores: &Ignores) -> bool {
        let center = matrix.get(x, y);
        match self {
            AutotilerMask::Padding => {
                let left = matrix.get_or(x as i32 - 2, y as i32, center);
                let right = matrix.get_or(x as i32 + 2, y as i32, center);
                let up = matrix.get_or(x as i32, y as i32 - 2, center);
                let down = matrix.get_or(x as i32, y as i32 + 2, center);

                let is_air = |x| x == AIR || (x != center && ignores.ignores(x));
                is_air(left) || is_air(right) || is_air(up) || is_air(down)
            }
            #[rustfmt::skip]
            #[allow(clippy::identity_op)]
            AutotilerMask::Pattern(pattern) => {
                       pattern[0][0].matches(center, matrix.get_or(x as i32  - 1, y as i32 - 1, center), ignores)
                    && pattern[0][1].matches(center, matrix.get_or(x as i32  + 0, y as i32 - 1, center), ignores)
                    && pattern[0][2].matches(center, matrix.get_or(x as i32  + 1, y as i32 - 1, center), ignores)
                    && pattern[1][0].matches(center, matrix.get_or(x as i32  - 1, y as i32 + 0, center), ignores)
                    && pattern[1][1].matches(center, matrix.get_or(x as i32  + 0, y as i32 + 0, center), ignores)
                    && pattern[1][2].matches(center, matrix.get_or(x as i32  + 1, y as i32 + 0, center), ignores)
                    && pattern[2][0].matches(center, matrix.get_or(x as i32  - 1, y as i32 + 1, center), ignores)
                    && pattern[2][1].matches(center, matrix.get_or(x as i32  + 0, y as i32 + 1, center), ignores)
                    && pattern[2][2].matches(center, matrix.get_or(x as i32  + 1, y as i32 + 1, center), ignores)
            },
            AutotilerMask::Center => true,
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
        if set.mask.validate(x, y, tiles, &tileset.ignores) {
            return Ok(Some(&set.tiles));
        }
    }

    Ok(None)
}
