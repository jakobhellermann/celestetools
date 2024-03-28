pub mod entity;
pub mod tileset;

use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    marker::PhantomData,
    ops::{BitOr, Sub},
    path::Path,
};

use anyhow::{anyhow, ensure, Context, Result};
use celesteloader::{
    atlas::Sprite,
    map::{utils::parse_map_name, Bounds, Decal, Map, Pos, Room},
    CelesteInstallation,
};
use tiny_skia::{
    BlendMode, Color, IntSize, Paint, PathBuilder, Pattern, Pixmap, PixmapRef,
    PremultipliedColorU8, Rect, Shader, Stroke, Transform,
};
use tracing::instrument;

use crate::asset::{AssetDb, LookupAsset, NullLookup, SpriteLocation};

use self::tileset::{tiles_to_matrix, tiles_to_matrix_scenery, Matrix, ParsedTileset};

#[derive(Clone, Copy)]
pub struct Layer(u8);
impl Layer {
    pub const NONE: Layer = Layer(0b00000000);
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
impl Sub for Layer {
    type Output = Layer;

    fn sub(self, rhs: Self) -> Self::Output {
        Layer(self.0 & !rhs.0)
    }
}

pub struct MapTileset {
    pub tileset_fg: HashMap<char, ParsedTileset>,
    pub tileset_bg: HashMap<char, ParsedTileset>,
}

impl MapTileset {
    pub fn vanilla(celeste: &CelesteInstallation) -> Result<Self> {
        let fgtiles_xml = celeste.read_to_string("Content/Graphics/ForegroundTiles.xml")?;
        let bgtiles_xml = celeste.read_to_string("Content/Graphics/BackgroundTiles.xml")?;
        Self::parse(&fgtiles_xml, &bgtiles_xml)
    }

    pub fn parse(fgtiles_xml: &str, bgtiles_xml: &str) -> Result<Self> {
        let tileset_fg = celesteloader::tileset::parse_tilesets(&fgtiles_xml)
            .context("error parsing fgtiles")?;
        let tileset_bg = celesteloader::tileset::parse_tilesets(&bgtiles_xml)
            .context("error parsing bgtiles")?;

        Ok(MapTileset {
            tileset_fg: ParsedTileset::parse(&tileset_fg)?,
            tileset_bg: ParsedTileset::parse(&tileset_bg)?,
        })
    }
}

pub struct CelesteRenderData {
    pub gameplay_sprites: HashMap<String, Sprite>,
    pub map_tileset: MapTileset,
    pub gameplay_atlas: Pixmap,
    pub scenery: Sprite,
}

impl CelesteRenderData {
    pub fn vanilla(celeste: &CelesteInstallation) -> Result<Self> {
        let mut base = CelesteRenderData::base(celeste)?;
        base.map_tileset = MapTileset::vanilla(celeste)?;
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
            map_tileset: MapTileset {
                tileset_fg: HashMap::new(),
                tileset_bg: HashMap::new(),
            },
            gameplay_atlas,
            gameplay_sprites,
            scenery,
        })
    }
}

pub struct RenderResult {
    pub image: Pixmap,
    pub bounds: Bounds,
    pub unknown_entities: BTreeMap<String, u32>,
}
impl RenderResult {
    /// Takes the image
    #[tracing::instrument(skip_all, fields(path = path.as_ref().to_str().unwrap_or("")))]
    pub fn save_png(
        &mut self,
        path: impl AsRef<Path>,
        compression: png::Compression,
    ) -> Result<(), png::EncodingError> {
        let file = File::create(path)?;
        self.encode_png(file, compression)
    }

    pub fn encode_png(
        &mut self,
        w: impl std::io::Write,
        compression: png::Compression,
    ) -> Result<(), png::EncodingError> {
        let mut image = std::mem::replace(&mut self.image, Pixmap::new(1, 1).unwrap());

        for pixel in image.pixels_mut() {
            let c = pixel.demultiply();
            // SAFETY: we just demultiplied
            *pixel = unsafe {
                PremultipliedColorU8::from_rgba(c.red(), c.green(), c.blue(), c.alpha())
                    .unwrap_unchecked()
            };
        }

        let mut encoder = png::Encoder::new(w, image.width(), image.height());
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_compression(compression);
        encoder.set_adaptive_filter(png::AdaptiveFilterType::Adaptive);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(image.data())?;

        Ok(())
    }
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
impl<'a> RenderMapSettings<'a> {
    pub fn include_room(self, f: &'a dyn Fn(&Room) -> bool) -> Self {
        RenderMapSettings {
            layer: self.layer,
            include_room: f,
        }
    }
}

#[instrument(skip_all, fields(name = map.package))]
pub fn render_with<L: LookupAsset>(
    render_data: &CelesteRenderData,
    asset_db: &mut AssetDb<L>,
    map: &Map,
    settings: RenderMapSettings,
) -> Result<RenderResult> {
    fastrand::seed(2);

    let parsed_map_name = parse_map_name(&map.package);

    let mut map_bounds = Bounds::empty();
    let mut rooms = Vec::new();
    for room in &map.rooms {
        if (settings.include_room)(room) {
            map_bounds = map_bounds.join(room.bounds);
            rooms.push(room);
        }
    }

    ensure!(!rooms.is_empty(), "No rooms to render");

    let pixmap = {
        let size_pixels = map_bounds.size.0 as usize * map_bounds.size.1 as usize;

        let data = {
            let _span = tracing::info_span!("allocate_pixmap").entered();
            allocate_data(size_pixels, [50, 50, 50, 255]).map_err(|_| {
                anyhow!(
                    "could not allocate {:.02}GiB",
                    size_pixels as f32 * 4.0 / (1024.0 * 1024.0 * 1024.0)
                )
            })?
        };

        Pixmap::from_vec(
            data,
            IntSize::from_wh(map_bounds.size.0, map_bounds.size.1).unwrap(),
        )
        .context("failed to create pixmap")?
    };

    let mut cx = RenderContext {
        map_bounds,
        pixmap,
        unknown_entities: Default::default(),
        area_id: parsed_map_name.order,
        _marker: PhantomData::<L>,
    };
    if parsed_map_name.name == "LostLevels" {
        cx.area_id = Some(10);
    }

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

struct RenderContext<L> {
    map_bounds: Bounds,
    pixmap: Pixmap,
    unknown_entities: BTreeMap<String, u32>,
    area_id: Option<u32>,
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

struct SpriteDesc {
    scale: (f32, f32),
    justify: (f32, f32),
    quad: Option<(i16, i16, i16, i16)>,
    tint: Option<Color>,
    rotation: f32,
}

impl Default for SpriteDesc {
    fn default() -> Self {
        Self {
            justify: (0.5, 0.5),
            scale: (1.0, 1.0),
            quad: None,
            tint: None,
            rotation: 0.0,
        }
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

    fn rect_inset(&mut self, inset: f32, map_pos: (f32, f32), size: (f32, f32), color: Color) {
        let (rect_x, rect_y) = self.transform_pos_f32((map_pos.0 + inset, map_pos.1 + inset));
        self.rect(
            Rect::from_xywh(rect_x, rect_y, size.0 - (2. * inset), size.1 - (2. * inset)).unwrap(),
            color,
            BlendMode::SourceOver,
        );
    }

    fn rect(&mut self, rect: Rect, color: Color, blend_mode: BlendMode) {
        self.pixmap.fill_rect(
            rect,
            &Paint {
                shader: tiny_skia::Shader::SolidColor(color),
                anti_alias: false,
                blend_mode,

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
        .unwrap_or(rect);

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

    fn sprite(
        &mut self,
        cx: &CelesteRenderData,
        map_pos: (f32, f32),
        sprite: SpriteLocation,
        desc: SpriteDesc,
    ) -> Result<()> {
        let SpriteDesc {
            scale,
            justify,
            quad,
            tint,
            rotation,
        } = desc;

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

        let justify_offset_x = (real_w as f32 * justify.0 + sprite_offset_x as f32) * scale.0;
        let justify_offset_y = (real_h as f32 * justify.1 + sprite_offset_y as f32) * scale.1;
        let draw_x = (x - justify_offset_x).floor();
        let draw_y = (y - justify_offset_y).floor();

        let pattern_transform = match sprite {
            SpriteLocation::Atlas(sprite) => Transform::from_translate(
                draw_x - sprite.x as f32 - quad_x as f32,
                draw_y - sprite.y as f32 - quad_y as f32,
            ),
            SpriteLocation::Raw(_) => Transform::from_translate(draw_x, draw_y),
        };

        let scale_transform = Transform::from_translate(-draw_x, -draw_y)
            .post_translate(-justify_offset_x, -justify_offset_y)
            .post_rotate(rotation.to_degrees())
            .post_translate(justify_offset_x, justify_offset_y)
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

        // TODO: tint should only tint the sprite itself
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

    fn tile_sprite(
        &mut self,
        atlas: PixmapRef,
        pos: Pos,
        atlas_position: (i16, i16),
        tint: Option<Color>,
    ) {
        let (x, y) = self.transform_pos(pos);

        let rect = Rect::from_xywh(x as f32, y as f32, 8.0, 8.0).unwrap();

        let pattern_transform = Transform::from_translate(
            (x - atlas_position.0 as i32) as f32,
            (y - atlas_position.1 as i32) as f32,
        );
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
            Transform::identity(),
            None,
        );

        if let Some(tint) = tint {
            self.pixmap.fill_rect(
                rect,
                &Paint {
                    shader: Shader::SolidColor(tint),
                    blend_mode: tiny_skia::BlendMode::Multiply,
                    anti_alias: false,
                    ..Default::default()
                },
                Transform::identity(),
                None,
            );
        }
    }

    #[instrument(skip_all, fields(name = room.name))]
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
            self.render_tileset(room, &bgtiles, &cx.map_tileset.tileset_bg, cx, asset_db)?;
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
            self.render_tileset(room, &fgtiles, &cx.map_tileset.tileset_fg, cx, asset_db)?;
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
        let tile_pos = room.bounds.position;
        self.render_tileset_inner(
            room.bounds.size_tiles(),
            tile_pos,
            tiles,
            tilesets,
            cx,
            asset_db,
        )
    }

    #[instrument(skip_all)]
    fn render_tileset_inner(
        &mut self,
        size: (u32, u32),
        tile_pos: Pos,
        tiles: &Matrix<char>,
        tilesets: &HashMap<char, ParsedTileset>,
        cx: &CelesteRenderData,
        asset_db: &mut AssetDb<L>,
    ) -> Result<()> {
        let (w, h) = size;

        for x in 0..w {
            for y in 0..h {
                let c = tiles.get(x, y);

                if c == '0' {
                    continue;
                }

                let tileset = tilesets
                    .get(&c)
                    .ok_or_else(|| anyhow!("tileset for '{}' not found", c))?;

                let random_tiles = tileset::choose_tile(tileset, x, y, &tiles)?.unwrap();
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

                self.tile_sprite(
                    atlas,
                    tile_pos.offset_tile(x as i32, y as i32),
                    sprite_pos,
                    None,
                );
            }
        }

        Ok(())
    }

    #[instrument(skip_all)]
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
                self.tile_sprite(
                    cx.gameplay_atlas.as_ref(),
                    tile_pos,
                    (sprite_x, sprite_y),
                    None,
                );
            }
        }

        Ok(())
    }

    #[instrument(skip_all)]
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
                sprite,
                SpriteDesc {
                    scale: (decal.scale_x, decal.scale_y),
                    ..Default::default()
                },
            )?;
        }

        Ok(())
    }

    #[instrument(skip_all)]
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
            if !entity::render_entity(self, fgtiles, cx, asset_db, room, e)
                .with_context(|| format!("couldn't render entity {}", e.name))?
            {
                *self.unknown_entities.entry(e.name.clone()).or_default() += 1;
            }
        }

        Ok(())
    }
}

pub fn allocate_data(
    size_pixels: usize,
    default_color_premultiplied: [u8; 4],
) -> Result<Vec<u8>, ()> {
    use std::alloc::{alloc, Layout};

    let size_bytes = size_pixels * 4;

    assert!(size_bytes > 0);
    unsafe {
        // SAFETY: layout has non-zero size
        let allocation = alloc(Layout::from_size_align(size_bytes, 4).unwrap()) as *mut [u8; 4];
        if allocation.is_null() {
            return Err(());
        }

        for i in 0..size_pixels {
            // SAFETY: inbounds, aligned, valid for write
            allocation.add(i).write(default_color_premultiplied);
        }

        // SAFETY: global allocator, u8 has align 1, size matches, length=capacity
        Ok(Vec::from_raw_parts(
            allocation.cast::<u8>(),
            size_bytes,
            size_bytes,
        ))
    }
}
