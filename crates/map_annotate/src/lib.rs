#![feature(iter_next_chunk, array_windows)]

use std::{
    ffi::OsStr,
    fs::File,
    io::BufWriter,
    ops::Range,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context, Result};
use image::{DynamicImage, ImageOutputFormat, Rgba};
use imageproc::drawing::{text_size, Canvas};
use rusttype::{Font, Scale};

const CONNECTION_COLOR_ANITIALIASING: bool = false;
const CONNECTION_COLOR_TRANSPARENCY: u8 = 255;
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

struct MapBounds {
    x: Range<i32>,
    y: Range<i32>,
}
impl MapBounds {
    fn from_pos_width(top_left: (i32, i32), size_px: (u32, u32)) -> Self {
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
    fn map_offset_f32(&self, point: (f32, f32)) -> (f32, f32) {
        (point.0 - self.x.start as f32, point.1 - self.y.start as f32)
    }
}

pub struct Annotate {
    map: DynamicImage,
    bounds: MapBounds,
    font: Font<'static>,
}
impl Annotate {
    pub fn map(path: impl AsRef<Path>, anchor: Anchor) -> Result<Self> {
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

        let font_data: &[u8] = include_bytes!("../DejaVuSans.ttf");
        let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();

        Ok(Annotate { map, bounds, font })
    }

    pub fn annotate_entries(&mut self, path: impl AsRef<Path>) -> Result<&mut Self> {
        let mut maps = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(path)?;
        for record in maps.records() {
            let record = record?;

            let [num, _name, x, y] = record.iter().next_chunk::<4>().unwrap();
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
                &self.font,
                num,
            );
        }

        Ok(self)
    }

    pub fn annotate_recent_cct_recordings(
        &mut self,
        include: impl Fn(&str) -> bool,
    ) -> Result<&mut Self> {
        let recent_recordings = PathBuf::from("C:/Program Files (x86)/Steam/steamapps/common/Celeste/ConsistencyTracker/physics-recordings/recent-recordings");

        for child in recent_recordings.read_dir()? {
            let child = child?;

            if let Some(filename) = child
                .path()
                .file_name()
                .and_then(OsStr::to_str)
                .and_then(|str| str.strip_suffix("_room-layout.json"))
            {
                let i: u32 = filename.parse()?;

                let json = std::fs::read_to_string(child.path())?;
                let json = serde_json::Value::from_str(&json)?;
                let chapter_name = json["chapterName"].as_str().unwrap();

                if !include(chapter_name) {
                    continue;
                }

                let position_log = child.path().with_file_name(format!("{i}_position-log.txt"));
                self.annotate_cct_recording(&position_log)?;
            }
        }

        Ok(self)
    }

    pub fn annotate_cct_recording(&mut self, position_log: &Path) -> Result<&mut Self> {
        let mut reader = csv::ReaderBuilder::new()
            .flexible(true)
            .from_path(&position_log)
            .with_context(|| format!("failed to read {}", position_log.display()))?;

        let mut path = Vec::new();
        for val in reader.records() {
            let val = val?;
            let [_frame, _frame_rta, x, y, _speed_x, _speed_y, _vel_x, _vel_y, _liftboost_x, _listboost_y, _retained, _stamina, flags] =
                val.iter().next_chunk().unwrap();
            let x: f32 = x.parse()?;
            let y: f32 = y.parse()?;

            let state = flags.split(' ').next().unwrap().to_owned();

            let (x, y) = self.bounds.map_offset_f32((x, y));

            let new_entry = (x, y, state);

            let same_as_last = path.last() == Some(&new_entry);
            if !same_as_last {
                path.push(new_entry);
            }
        }

        for &[(from_x, from_y, ref state), (to_x, to_y, _)] in path.array_windows() {
            let color = match state.as_str() {
                "StNormal" => Rgba([0, 255, 0, CONNECTION_COLOR_TRANSPARENCY]),
                "StDash" => Rgba([255, 0, 0, CONNECTION_COLOR_TRANSPARENCY]),
                _ => Rgba([255, 0, 255, CONNECTION_COLOR_TRANSPARENCY]),
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
    let size = text_size(scale, &font, text);
    imageproc::drawing::draw_text_mut(
        canvas,
        color,
        position.0 - size.0 / 2,
        position.1 - size.1 / 2,
        scale,
        &font,
        text,
    );
}
