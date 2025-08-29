#![allow(clippy::wildcard_in_or_patterns)]
use std::{fs::File, io::BufWriter, path::Path};

use ab_glyph::Font;
use anyhow::Result;
use celesteloader::{
    cct_physics_inspector::{MapBounds, PhysicsInspector},
    map::Bounds,
};
use image::{DynamicImage, ImageFormat, Rgba};
use imageproc::drawing::{text_size, Canvas};
use tiny_skia::{
    Color, GradientStop, LinearGradient, Paint, PathBuilder, Pixmap, Point, Shader, Stroke,
    Transform,
};

const CONNECTION_COLOR_ANITIALIASING: bool = false;
const CONNECTION_COLOR_TRANSPARENCY: u8 = 100;

pub struct Annotate {
    map: DynamicImage,
    pub bounds: MapBounds,
}
impl Annotate {
    pub fn new(map: DynamicImage, bounds: MapBounds) -> Self {
        assert_eq!(map.dimensions(), bounds.dimensions());

        Annotate { map, bounds }
    }

    pub fn load(path: impl AsRef<Path>, anchor: Anchor) -> Result<Self> {
        let map = image::ImageReader::open(path)?.decode()?;

        let map_dims = map.dimensions();
        let bounds = match anchor {
            Anchor::TopLeft { room_pos } => MapBounds::from_pos_width(room_pos, map_dims),
            Anchor::BottomLeft {
                room_pos,
                room_height,
            } => {
                let bottom_y = room_pos.1 + room_height;
                MapBounds {
                    x: room_pos.0..room_pos.0 + map_dims.0 as i32,
                    y: bottom_y - map_dims.1 as i32..bottom_y,
                }
            }
        };

        Ok(Annotate { map, bounds })
    }

    pub fn annotate_entries(
        &mut self,
        path: impl AsRef<Path>,
        font: &impl Font,
    ) -> Result<&mut Self> {
        let circle_radius = 22;
        let mut maps = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(path)?;
        for record in maps.records() {
            let record = record?;

            let [num, name, x, y] = record.iter().collect::<Vec<_>>().try_into().unwrap();
            let x: i32 = x.parse()?;
            let y: i32 = y.parse()?;

            let position = self.bounds.map_offset((x, y));

            let bench_name = name.strip_prefix("bench_");

            let (name, color) = match bench_name {
                Some(name) => (name, Rgba([0, 0, 255, 255])),
                _ => (num, Rgba([255, 0, 0, 255])),
            };

            imageproc::drawing::draw_filled_circle_mut(
                &mut self.map,
                position,
                circle_radius,
                color,
            );
            let scale = 35.0;
            draw_text_centered(
                &mut self.map,
                Rgba([255, 255, 255, 255]),
                position,
                scale,
                font,
                name,
            );
        }

        Ok(self)
    }

    pub fn annotate_cct_recording(
        &mut self,
        physics_inspector: &PhysicsInspector,
        i: u32,
    ) -> Result<&mut Self> {
        let position_log = physics_inspector.position_log(i)?;

        let mut path = Vec::new();
        for log in position_log {
            let item = log?;

            let state = item.flags.split(' ').next().unwrap().to_owned();
            let (map_x, map_y) = self.bounds.map_offset_f32((item.x, item.y));

            let new_entry = (map_x, map_y, state);
            let same_as_last = path.last() == Some(&new_entry);
            if !same_as_last {
                path.push(new_entry);
            }
        }

        for window in path.windows(2) {
            let &[(from_x, from_y, ref state), (to_x, to_y, _)] = window else {
                unreachable!()
            };

            let color = match state.as_str() {
                "StNormal" => Rgba([0, 255, 0, CONNECTION_COLOR_TRANSPARENCY]),
                "StDash" => Rgba([255, 0, 0, CONNECTION_COLOR_TRANSPARENCY]),
                "StClimb" => Rgba([255, 255, 0, 200]),
                "StDummy" => Rgba([255, 255, 255, CONNECTION_COLOR_TRANSPARENCY]),
                "StOther" | _ => Rgba([255, 0, 255, CONNECTION_COLOR_TRANSPARENCY]),
            };

            if CONNECTION_COLOR_ANITIALIASING {
                imageproc::drawing::draw_antialiased_line_segment_mut(
                    &mut self.map,
                    (from_x as i32, from_y as i32),
                    (to_x as i32, to_y as i32),
                    color,
                    imageproc::pixelops::interpolate,
                );
            } else {
                imageproc::drawing::draw_line_segment_mut(
                    &mut self.map,
                    (from_x, from_y),
                    (to_x, to_y),
                    color,
                );
            }
        }

        Ok(self)
    }

    pub fn save(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let out = File::create(path)?;
        self.map
            .write_to(&mut BufWriter::new(out), ImageFormat::Png)?;

        Ok(())
    }
}

pub enum Anchor {
    TopLeft {
        room_pos: (i32, i32),
    },
    BottomLeft {
        room_pos: (i32, i32),
        room_height: i32,
    },
}

fn draw_text_centered(
    canvas: &mut DynamicImage,
    color: <DynamicImage as Canvas>::Pixel,
    position: (i32, i32),
    scale: f32,
    font: &impl Font,
    text: &str,
) {
    let size = text_size(scale, font, text);
    imageproc::drawing::draw_text_mut(
        canvas,
        color,
        position.0 - size.0 as i32 / 2,
        position.1 - size.1 as i32 / 2,
        scale,
        font,
        text,
    );
}

#[derive(Clone, Copy)]
pub enum ColorMode {
    Gradient,
    State,
    Random,
    Color([u8; 4]),
}

#[derive(Clone, Copy)]
pub struct LineSettings {
    pub width: f32,
    pub anti_alias: bool,
    pub color_mode: ColorMode,
}
impl Default for LineSettings {
    fn default() -> Self {
        Self {
            width: 2.0,
            anti_alias: true,
            color_mode: ColorMode::Gradient,
        }
    }
}

pub fn annotate_cct_recording_skia(
    image: &mut Pixmap,
    physics_inspector: &PhysicsInspector,
    i: impl Iterator<Item = u32>,
    bounds: Bounds,
    settings: LineSettings,
) -> Result<()> {
    let mut random_color_index = 0;
    let random_transparency = 200;
    let random_colors = [
        Color::from_rgba8(255, 0, 0, random_transparency),
        Color::from_rgba8(0, 255, 0, random_transparency),
        Color::from_rgba8(0, 0, 255, random_transparency),
        Color::from_rgba8(255, 255, 0, random_transparency),
        Color::from_rgba8(255, 0, 255, random_transparency),
        Color::from_rgba8(0, 255, 255, random_transparency),
        Color::from_rgba8(128, 0, 128, random_transparency),
        Color::from_rgba8(255, 165, 0, random_transparency),
        Color::from_rgba8(0, 128, 0, random_transparency),
        Color::from_rgba8(255, 192, 203, random_transparency),
    ];

    for i in i {
        annotate_single_cct_recording_skia(
            image,
            physics_inspector,
            i,
            bounds,
            settings,
            random_colors[random_color_index],
        )?;

        random_color_index = (random_color_index + 1) % random_colors.len();
    }
    Ok(())
}

fn annotate_single_cct_recording_skia(
    image: &mut Pixmap,
    physics_inspector: &PhysicsInspector,
    i: u32,
    bounds: Bounds,
    settings: LineSettings,
    random_color: Color,
) -> Result<()> {
    // read path
    let position_log = physics_inspector.position_log(i)?;

    let mut path = Vec::new();
    for log in position_log {
        let item = log?;
        let state = item.flags.split(' ').next().unwrap().to_owned();

        let new_entry = (item.x, item.y, state);
        let same_as_last = path.last() == Some(&new_entry);
        if !same_as_last {
            path.push(new_entry);
        }
    }

    if path.len() <= 1 {
        return Ok(());
    }

    let map2img = Transform::from_translate(-bounds.position.x as f32, -bounds.position.y as f32);

    // render fn
    let mut flush = |pb: PathBuilder, state: &str| {
        let Some(path) = pb.finish() else { return };

        let shader = match settings.color_mode {
            ColorMode::Gradient => LinearGradient::new(
                Point::from_xy(0.0, 0.0),
                Point::from_xy(bounds.size.0 as f32, bounds.size.1 as f32),
                vec![
                    GradientStop::new(0.0, Color::from_rgba8(255, 0, 0, 255)),
                    GradientStop::new(0.5, Color::from_rgba8(128, 0, 128, 255)),
                    GradientStop::new(1.0, Color::from_rgba8(15, 30, 150, 255)),
                ],
                tiny_skia::SpreadMode::Reflect,
                Transform::identity(),
            )
            .unwrap(),
            ColorMode::State => {
                let transparency = 255;

                let color = match state {
                    "StNormal" => Color::from_rgba8(0, 255, 0, transparency),
                    "StDash" => Color::from_rgba8(255, 0, 0, transparency),
                    "StClimb" => Color::from_rgba8(255, 255, 0, transparency),
                    "StDummy" => Color::from_rgba8(255, 255, 255, transparency),
                    "StOther" | _ => Color::from_rgba8(255, 0, 255, transparency),
                };
                Shader::SolidColor(color)
            }
            ColorMode::Color([r, g, b, a]) => Shader::SolidColor(Color::from_rgba8(r, g, b, a)),
            ColorMode::Random => Shader::SolidColor(random_color),
        };

        image.stroke_path(
            &path,
            &Paint {
                shader,
                blend_mode: tiny_skia::BlendMode::SourceOver,
                anti_alias: settings.anti_alias,
                ..Default::default()
            },
            &Stroke {
                width: settings.width,
                line_cap: tiny_skia::LineCap::Butt,
                line_join: tiny_skia::LineJoin::Round,
                ..Default::default()
            },
            map2img,
            None,
        );
    };

    // iterate through path
    let mut pb = PathBuilder::new();

    let mut items = path.into_iter();
    let (x, y, mut last_state) = items.next().unwrap();
    pb.move_to(x, y);

    for (x, y, state) in items {
        pb.line_to(x, y);

        if let ColorMode::State = settings.color_mode {
            if state != last_state {
                flush(std::mem::take(&mut pb), &last_state);
                pb.move_to(x, y)
            }
        }

        last_state = state;
    }

    flush(pb, &last_state);

    Ok(())
}
