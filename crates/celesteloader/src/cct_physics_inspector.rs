use crate::{map::Bounds, CelesteInstallation};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::{
    ffi::OsStr,
    io::BufReader,
    ops::Range,
    path::{Path, PathBuf},
};

impl From<Bounds> for MapBounds {
    fn from(value: Bounds) -> Self {
        MapBounds::from_pos_width((value.position.x, value.position.y), value.size)
    }
}

// TODO: replace with Bounds
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

    pub fn map_offset(&self, point: (i32, i32)) -> (i32, i32) {
        (point.0 - self.x.start, point.1 - self.y.start)
    }
    pub fn map_offset_f32(&self, point: (f32, f32)) -> (f32, f32) {
        (point.0 - self.x.start as f32, point.1 - self.y.start as f32)
    }
}

fn remap(val: i32, from: Range<i32>, to: Range<i32>) -> f32 {
    to.start as f32
        + (to.end - to.start) as f32 * ((val - from.start) as f32 / (from.end - from.start) as f32)
}

#[derive(Clone)]
pub struct PhysicsInspector {
    pub recent_recordings: PathBuf,
}

impl PhysicsInspector {
    pub fn new(installation: &CelesteInstallation) -> Self {
        let recent_recordings = installation
            .path
            .join("ConsistencyTracker/physics-recordings/recent-recordings");

        PhysicsInspector { recent_recordings }
    }
    pub fn recent_recordings(&self) -> Result<Vec<(u32, CCTRoomLayout)>, anyhow::Error> {
        let mut items = Vec::new();

        for child in self
            .recent_recordings
            .read_dir()
            .context("failed to read recent physics inspector logs")?
        {
            let child = child?;

            if let Some(filename) = child
                .path()
                .file_name()
                .and_then(OsStr::to_str)
                .and_then(|str| str.strip_suffix("_room-layout.json"))
            {
                let i: u32 = filename.parse()?;

                let room_layout =
                    CCTRoomLayout::from_reader(BufReader::new(std::fs::File::open(child.path())?))?;

                items.push((i, room_layout));
            }
        }

        Ok(items)
    }

    pub fn delete_recent_recordings(&self) -> Result<()> {
        for entry in std::fs::read_dir(&self.recent_recordings)? {
            let entry = entry?;
            let path = entry.path();
            let delete = entry.path().to_str().map_or(false, |path| {
                path.ends_with("_room-layout.json") || path.ends_with("_position-log.txt")
            });

            if delete {
                std::fs::remove_file(path)?;
            }
        }

        Ok(())
    }

    pub fn position_log(&self, i: u32) -> Result<impl Iterator<Item = Result<(f32, f32, String)>>> {
        let path = self.recent_recordings.join(format!("{i}_position-log.txt"));
        let reader = csv::ReaderBuilder::new()
            .flexible(true)
            .from_path(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;

        Ok(reader.into_records()
            .map(|record| -> anyhow::Result<_>{
                let record = record?;
                let [_frame, _frame_rta, x, y, _speed_x, _speed_y, _vel_x, _vel_y, _liftboost_x, _listboost_y, _retained, _stamina, flags] =
                    record.iter().collect::<Vec<_>>()[0..13].try_into().unwrap();
                let x: f32 = x.parse()?;
                let y: f32 = y.parse()?;

                Ok((x,y,flags.to_owned()))
            }))
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CCTRoomLayout {
    pub id: u32,
    pub name: Option<String>,
    #[serde(rename = "SID")]
    pub sid: Option<String>,
    pub chapter_name: String,
    pub side_name: String,
    pub frame_count: u32,
    pub recording_started: String,
    pub rooms: Vec<CCTRoom>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CCTRoom {
    pub debug_room_name: String,
    pub level_bounds: CCTLevelBounds,
    // pub solid_tiles: Vec<Vec<u8>>,
}

#[derive(Deserialize, Debug)]
pub struct CCTLevelBounds {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl CCTRoomLayout {
    pub fn from_reader(reader: impl std::io::Read) -> Result<Self, serde_json::Error> {
        serde_json::from_reader::<_, Self>(reader)
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, anyhow::Error> {
        let reader = BufReader::new(std::fs::File::open(path.as_ref())?);
        let room = CCTRoomLayout::from_reader(reader)?;
        Ok(room)
    }

    pub fn bounds(&self) -> MapBounds {
        self.rooms
            .iter()
            .map(|room| {
                MapBounds::xywh(
                    room.level_bounds.x as i32,
                    room.level_bounds.y as i32,
                    room.level_bounds.w as i32,
                    room.level_bounds.h as i32,
                )
            })
            .reduce(MapBounds::join)
            .unwrap()
    }
}
