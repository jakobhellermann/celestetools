#![allow(dead_code)]

use crate::{
    asset::{AssetDb, LookupAsset, SpriteLocation},
    rendering::{tileset::Matrix, RenderContext, SpriteDesc},
    CelesteRenderData,
};
use anyhow::Result;

#[derive(Clone, Copy)]
pub struct Border {
    pub left: i16,
    pub right: i16,
    pub top: i16,
    pub bottom: i16,
}

pub struct NinePatchOptions {
    pub tile_size: i16,
    pub tile_width: Option<i16>,
    pub tile_height: Option<i16>,
    pub border: Option<Border>,
    pub mode: NinePatchMode,
    pub border_mode: NinePatchBorderMode,
    pub fill_mode: NinePatchFillMode,
    pub use_real_size: bool,
    pub hide_overflow: bool,
    pub color: Option<tiny_skia::Color>,
}
impl NinePatchOptions {
    pub fn border() -> Self {
        NinePatchOptions {
            mode: NinePatchMode::Border,
            ..Default::default()
        }
    }
}
#[derive(Clone, Copy)]
pub enum NinePatchMode {
    Border,
    Fill,
}
#[derive(Clone, Copy)]
pub enum NinePatchBorderMode {
    Repeat,
}
#[derive(Clone, Copy)]
pub enum NinePatchFillMode {
    Repeat,
    Random,
}
impl Default for NinePatchOptions {
    fn default() -> Self {
        Self {
            tile_size: 8,
            tile_width: None,
            tile_height: None,
            border: None,
            mode: NinePatchMode::Fill,
            border_mode: NinePatchBorderMode::Repeat,
            fill_mode: NinePatchFillMode::Repeat,
            use_real_size: false,
            hide_overflow: true,
            color: None,
        }
    }
}

pub fn nine_patch<L: LookupAsset>(
    asset_db: &mut AssetDb<L>,
    cx: &CelesteRenderData,
    r: &mut RenderContext<L>,
    texture: &str,
    draw_pos: (f32, f32),
    draw_size: (i16, i16),
    options: NinePatchOptions,
) -> Result<()> {
    let sprite = asset_db.lookup_gameplay(cx, texture)?;
    let tile_width = options.tile_width.unwrap_or(options.tile_size);
    let tile_height = options.tile_height.unwrap_or(options.tile_size);

    let sprite_size = if options.use_real_size {
        (sprite.real_width(), sprite.real_height())
    } else {
        (sprite.width(), sprite.height())
    };
    let w_tiles = (sprite_size.0 as u16).div_ceil(tile_width as u16) as i16;
    let h_tiles = (sprite_size.1 as u16).div_ceil(tile_height as u16) as i16;

    let matrix = Matrix::from_fn(w_tiles as u32, h_tiles as u32, |_x, _y| {
        // getRelativeQuad(x-1)*tile_width, (y-1)*tile_height), tile_width, tile_height, hide_overflow, use_real_size)
    });

    let border = options.border.unwrap_or(Border {
        left: tile_width,
        right: tile_width,
        top: tile_height,
        bottom: tile_height,
    });

    let draw_border = matches!(options.mode, NinePatchMode::Border | NinePatchMode::Fill);
    let draw_middle = matches!(options.mode, NinePatchMode::Fill);

    if draw_border {
        draw_corner_quads(
            r,
            cx,
            sprite,
            draw_pos,
            draw_size,
            &matrix,
            sprite_size,
            border,
            &options,
        )?;
        draw_edge_quads(
            r,
            cx,
            sprite,
            draw_pos,
            draw_size,
            &matrix,
            sprite_size,
            border,
            &options,
        )?;
    }
    if draw_middle {
        draw_middle_quads(
            r,
            cx,
            sprite,
            draw_pos,
            draw_size,
            &matrix,
            sprite_size,
            border,
            &options,
        )?;
    }

    Ok(())
}

fn draw_corner_quads<L: LookupAsset>(
    r: &mut RenderContext<L>,
    cx: &CelesteRenderData,
    sprite: SpriteLocation,
    draw_pos: (f32, f32),
    draw_size: (i16, i16),
    _matrix: &Matrix<()>,
    sprite_size: (i16, i16),
    border: Border,
    options: &NinePatchOptions,
) -> Result<()> {
    let offset_x = (draw_size.0 - border.right) as f32;
    let offset_y = (draw_size.1 - border.bottom) as f32;

    // TODO hideoverflow realsize

    // top left
    if draw_size.0 > 0 && draw_size.1 > 0 && border.left > 0 && border.top > 0 {
        let quad = (0, 0, border.left, border.top);
        r.sprite(
            cx,
            draw_pos,
            sprite,
            SpriteDesc {
                justify: (0.0, 0.0),
                quad: Some(quad),
                tint: options.color,
                ..Default::default()
            },
        )?;
    }
    // top right
    if draw_size.0 > border.left && draw_size.1 >= 0 && border.right > 0 && border.top > 0 {
        let quad = (sprite_size.0 - border.right, 0, border.right, border.top);
        r.sprite(
            cx,
            (draw_pos.0 + offset_x, draw_pos.1),
            sprite,
            SpriteDesc {
                justify: (0.0, 0.0),
                quad: Some(quad),
                tint: options.color,
                ..Default::default()
            },
        )?;
    }
    // bottom left
    if draw_size.0 > 0 && draw_size.1 > border.bottom {
        let quad = (0, sprite_size.1 - border.bottom, border.left, border.bottom);
        r.sprite(
            cx,
            (draw_pos.0, draw_pos.1 + offset_y),
            sprite,
            SpriteDesc {
                justify: (0.0, 0.0),
                quad: Some(quad),
                tint: options.color,
                ..Default::default()
            },
        )?;
    }
    // bottom right
    if draw_size.0 > border.right && draw_size.1 > border.bottom {
        let quad = (
            sprite_size.1 - border.right,
            sprite_size.1 - border.bottom,
            border.left,
            border.bottom,
        );
        r.sprite(
            cx,
            (draw_pos.0 + offset_x, draw_pos.1 + offset_y),
            sprite,
            SpriteDesc {
                justify: (0.0, 0.0),
                quad: Some(quad),
                tint: options.color,
                ..Default::default()
            },
        )?;
    }

    Ok(())
}

fn draw_edge_quads<L: LookupAsset>(
    r: &mut RenderContext<L>,
    cx: &CelesteRenderData,
    sprite: SpriteLocation,
    draw_pos: (f32, f32),
    draw_size: (i16, i16),
    _matrix: &Matrix<()>,
    sprite_size: (i16, i16),
    border: Border,
    options: &NinePatchOptions,
) -> Result<()> {
    let repeat_mode = options.fill_mode;

    let opposite_offset_x = draw_size.0 - border.right;
    let opposite_offset_y = draw_size.1 - border.bottom;

    match repeat_mode {
        NinePatchFillMode::Repeat => {
            let width_no_border = sprite_size.0 - border.left - border.right;
            let height_no_border = sprite_size.1 - border.top - border.bottom;

            let (mut processed_x, mut processed_y) = (border.left, border.top);

            // vertical
            while processed_y < draw_size.1 - border.right {
                let quad_height = (draw_size.1 - border.bottom - processed_y).min(height_no_border);
                let quad_left = (
                    0,
                    border.top,
                    border.left,
                    quad_height,
                    // hide_overflow,
                    // real_size,
                );
                r.sprite(
                    cx,
                    (draw_pos.0, draw_pos.1 + processed_y as f32),
                    sprite,
                    SpriteDesc {
                        justify: (0.0, 0.0),
                        quad: Some(quad_left),
                        tint: options.color,
                        ..Default::default()
                    },
                )?;
                let quad_right = (
                    sprite_size.0 - border.right,
                    border.bottom,
                    border.right,
                    quad_height,
                    // hide_overflow,
                    // real_size,
                );
                r.sprite(
                    cx,
                    (
                        draw_pos.0 + opposite_offset_x as f32,
                        draw_pos.1 + processed_y as f32,
                    ),
                    sprite,
                    SpriteDesc {
                        justify: (0.0, 0.0),
                        quad: Some(quad_right),
                        tint: options.color,
                        ..Default::default()
                    },
                )?;

                processed_y += height_no_border;
            }

            // horizontal
            while processed_x < draw_size.0 - border.bottom {
                let quad_width = (draw_size.0 - border.right - processed_x).min(width_no_border);

                let quad_top = (
                    border.left,
                    0,
                    quad_width,
                    border.top,
                    // hide_overflow,
                    // real_size,
                );
                r.sprite(
                    cx,
                    (draw_pos.0 + processed_x as f32, draw_pos.1),
                    sprite,
                    SpriteDesc {
                        justify: (0.0, 0.0),
                        quad: Some(quad_top),
                        tint: options.color,
                        ..Default::default()
                    },
                )?;
                let quad_bottom = (
                    border.right,
                    sprite_size.1 - border.bottom,
                    quad_width,
                    border.bottom,
                    // hide_overflow,
                    // real_size,
                );
                r.sprite(
                    cx,
                    (
                        draw_pos.0 + processed_x as f32,
                        draw_pos.1 + opposite_offset_y as f32,
                    ),
                    sprite,
                    SpriteDesc {
                        justify: (0.0, 0.0),
                        quad: Some(quad_bottom),
                        tint: options.color,
                        ..Default::default()
                    },
                )?;

                processed_x += width_no_border;
            }
        }
        NinePatchFillMode::Random => todo!(),
    }

    Ok(())
}

fn draw_middle_quads<L: LookupAsset>(
    r: &mut RenderContext<L>,
    cx: &CelesteRenderData,
    sprite: SpriteLocation,
    draw_pos: (f32, f32),
    draw_size: (i16, i16),
    _matrix: &Matrix<()>,
    sprite_size: (i16, i16),
    border: Border,
    options: &NinePatchOptions,
) -> Result<()> {
    let repeat_mode = options.fill_mode;

    match repeat_mode {
        NinePatchFillMode::Repeat => {
            let width_no_border = sprite_size.0 - border.left - border.right;
            let height_no_border = sprite_size.1 - border.top - border.bottom;

            let (mut processed_x, mut processed_y) = (border.left, border.top);

            while processed_y < draw_size.1 - border.bottom {
                while processed_x < draw_size.0 - border.right {
                    let quad_width =
                        (draw_size.0 - border.right - processed_x).min(width_no_border);
                    let quad_height =
                        (draw_size.1 - border.bottom - processed_y).min(height_no_border);

                    let x = draw_pos.0 + processed_x as f32;
                    let y = draw_pos.1 + processed_y as f32;

                    let quad = (
                        border.left,
                        border.top,
                        quad_width,
                        quad_height,
                        // hide_overflow,
                        // real_size,
                    );
                    r.sprite(
                        cx,
                        (x, y),
                        sprite,
                        SpriteDesc {
                            justify: (0.0, 0.0),
                            quad: Some(quad),
                            tint: options.color,
                            ..Default::default()
                        },
                    )?;

                    processed_x += width_no_border;
                }
                processed_x = border.left;
                processed_y += height_no_border;
            }
        }
        NinePatchFillMode::Random => todo!(),
    }

    Ok(())
}
