pub mod cct_physics_inspector;

use std::{fs::File, io::BufWriter, ops::Range, path::Path};

use anyhow::Result;
use cct_physics_inspector::PhysicsInspector;
use image::{DynamicImage, ImageOutputFormat, Rgba};
use imageproc::drawing::{text_size, Canvas};
use rusttype::{Font, Scale};

const CONNECTION_COLOR_ANITIALIASING: bool = false;
const CONNECTION_COLOR_TRANSPARENCY: u8 = 100;
#[allow(unused)]
const CONNECTION_COLORS: &[Rgba<u8>] = &[
    Rgba([255, 0, 0, CONNECTION_COLOR_TRANSPARENCY]),
    Rgba([0, 255, 0, CONNECTION_COLOR_TRANSPARENCY]),
    Rgba([0, 0, 255, CONNECTION_COLOR_TRANSPARENCY]),
];

fn remap(val: i32, from: Range<i32>, to: Range<i32>) -> f32 {
    to.start as f32
        + (to.end - to.start) as f32 * ((val - from.start) as f32 / (from.end - from.start) as f32)
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MapBounds {
    pub x: Range<i32>,
    pub y: Range<i32>,
}

impl std::fmt::Display for MapBounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (w, h) = self.dimensions();
        write!(f, "{},{} {},{}", self.x.start, self.y.start, w, h)
    }
}

impl MapBounds {
    pub fn xywh(x: i32, y: i32, w: i32, h: i32) -> Self {
        MapBounds {
            x: x..x + w,
            y: y..y + h,
        }
    }
    #[allow(clippy::reversed_empty_ranges)]
    pub fn empty() -> Self {
        MapBounds {
            x: i32::MAX..i32::MIN,
            y: i32::MAX..i32::MIN,
        }
    }
    pub fn join(self, other: MapBounds) -> Self {
        let x = self.x.start.min(other.x.start)..self.x.end.max(other.x.end);
        let y = self.y.start.min(other.y.start)..self.y.end.max(other.y.end);
        MapBounds { x, y }
    }
    pub fn dimensions(&self) -> (u32, u32) {
        (
            (self.x.end - self.x.start) as u32,
            (self.y.end - self.y.start) as u32,
        )
    }

    pub fn from_pos_width(top_left: (i32, i32), size_px: (u32, u32)) -> Self {
        MapBounds {
            x: top_left.0..(top_left.0 + size_px.0 as i32),
            y: top_left.1..(top_left.1 + size_px.1 as i32),
        }
    }

    #[allow(dead_code)]
    fn map_to(&self, point: (i32, i32), x_range: Range<i32>, y_range: Range<i32>) -> (f32, f32) {
        let x = remap(point.0, self.x.clone(), x_range);
        let y = remap(point.1, self.y.clone(), y_range);
        (x, y)
    }

    fn map_offset(&self, point: (i32, i32)) -> (i32, i32) {
        (point.0 - self.x.start, point.1 - self.y.start)
    }
    pub fn map_offset_f32(&self, point: (f32, f32)) -> (f32, f32) {
        (point.0 - self.x.start as f32, point.1 - self.y.start as f32)
    }
}

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
        let map = image::io::Reader::open(path)?.decode()?;

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

    pub fn annotate_entries(&mut self, path: impl AsRef<Path>, font: &Font) -> Result<&mut Self> {
        let mut maps = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(path)?;
        for record in maps.records() {
            let record = record?;

            let [num, _name, x, y] = record.iter().collect::<Vec<_>>().try_into().unwrap();
            let x: i32 = x.parse()?;
            let y: i32 = y.parse()?;

            let position = self.bounds.map_offset((x, y));

            let scale = Scale::uniform(35.0);
            imageproc::drawing::draw_filled_circle_mut(
                &mut self.map,
                position,
                20,
                Rgba([255, 0, 0, 255]),
            );
            draw_text_centered(
                &mut self.map,
                Rgba([255, 255, 255, 255]),
                position,
                scale,
                font,
                num,
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
            let (x, y, flags) = log?;

            let state = flags.split(' ').next().unwrap().to_owned();
            let (map_x, map_y) = self.bounds.map_offset_f32((x, y));

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

            #[allow(clippy::wildcard_in_or_patterns)]
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
            .write_to(&mut BufWriter::new(out), ImageOutputFormat::Png)?;

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

fn draw_text_centered<'a>(
    canvas: &'a mut DynamicImage,
    color: <DynamicImage as Canvas>::Pixel,
    position: (i32, i32),
    scale: Scale,
    font: &'a Font<'a>,
    text: &str,
) {
    let size = text_size(scale, font, text);
    imageproc::drawing::draw_text_mut(
        canvas,
        color,
        position.0 - size.0 / 2,
        position.1 - size.1 / 2,
        scale,
        font,
        text,
    );
}
