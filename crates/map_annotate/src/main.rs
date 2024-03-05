#![feature(iter_next_chunk, array_windows)]

use std::{ffi::OsStr, fs::File, io::BufWriter, ops::Range, path::PathBuf, str::FromStr};

use anyhow::{Context, Result};
use image::{DynamicImage, ImageOutputFormat, Rgba};
use imageproc::drawing::{text_size, Canvas};
use rusttype::{Font, Scale};

const CHAPTER_NAME: &str = "Flowing Gallery";

const CONNECTION_COLOR_ANITIALIASING: bool = false;
const CONNECTION_COLOR_TRANSPARENCY: u8 = 255;
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
    fn from_pos_width(pos_px: (i32, i32), size_px: (u32, u32)) -> Self {
        MapBounds {
            x: pos_px.0..(pos_px.0 + size_px.0 as i32),
            y: pos_px.1..(pos_px.1 + size_px.1 as i32),
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

fn main() -> Result<()> {
    let font_data: &[u8] = include_bytes!("../DejaVuSans.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();

    let mut map = image::io::Reader::open("testing/annotate/map.png")?.decode()?;
    let mut maps = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("testing/annotate/maps.csv")?;

    let top_left = (87 * 8, -84 * 8);
    let bounds = MapBounds::from_pos_width(top_left, map.dimensions());

    for record in maps.records() {
        let record = record?;

        let [num, _name, x, y] = record.iter().next_chunk::<4>().unwrap();
        let x: i32 = x.parse()?;
        let y: i32 = y.parse()?;

        let position = bounds.map_offset((x, y));

        let scale = Scale::uniform(35.0);
        imageproc::drawing::draw_filled_circle_mut(&mut map, position, 20, Rgba([255, 0, 0, 255]));
        draw_text_centered(
            &mut map,
            Rgba([255, 255, 255, 255]),
            position,
            scale,
            &font,
            num,
        );
    }

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

            if chapter_name != CHAPTER_NAME {
                continue;
            }

            let position_log = child.path().with_file_name(format!("{i}_position-log.txt"));
            let mut reader = csv::ReaderBuilder::new()
                .flexible(true)
                .from_path(&position_log)
                .with_context(|| format!("failed to read {}", position_log.display()))?;

            let mut path: Vec<(f32, f32)> = Vec::new();
            for val in reader.records() {
                let val = val?;
                let [_frame, _frame_rta, x, y] = val.iter().next_chunk().unwrap();
                let x: f32 = x.parse()?;
                let y: f32 = y.parse()?;

                let new_entry = bounds.map_offset_f32((x, y));

                let same_as_last = path.last() == Some(&new_entry);
                if !same_as_last {
                    path.push(new_entry);
                }
            }

            let color = CONNECTION_COLORS[i as usize % CONNECTION_COLORS.len()];

            for &[from, to] in path.array_windows() {
                if CONNECTION_COLOR_ANITIALIASING {
                    imageproc::drawing::draw_antialiased_line_segment_mut(
                        &mut map,
                        (from.0 as i32, from.1 as i32),
                        (to.0 as i32, to.1 as i32),
                        color,
                        imageproc::pixelops::interpolate,
                    );
                } else {
                    imageproc::drawing::draw_line_segment_mut(&mut map, from, to, color);
                }
            }
        }
    }

    let out = File::create("out.png")?;
    map.write_to(&mut BufWriter::new(out), ImageOutputFormat::Png)?;

    Ok(())
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
