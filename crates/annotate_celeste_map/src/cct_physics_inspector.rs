use anyhow::{Context, Result};
use celesteloader::CelesteInstallation;
use serde::Deserialize;
use std::{
    ffi::OsStr,
    io::BufReader,
    path::{Path, PathBuf},
};

use crate::MapBounds;

pub struct PhysicsInspector {
    recent_recordings: PathBuf,
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
    pub chapter_name: String,
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
