//! WIP from a long time ago, not really that useful

use std::{
    fs::File,
    io::{BufReader, Read, Seek},
    path::Path,
};

use anyhow::{anyhow, ensure, Result};
use zip::ZipArchive;

fn main() -> Result<()> {
    let file = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("expected path to zip"))?;
    let mod_zip = Path::new(&file);
    let target = std::env::args()
        .nth(2)
        .ok_or_else(|| anyhow!("expected target folder as second argument"))?;
    let target = Path::new(&target);

    ensure!(
        mod_zip.extension().map_or(false, |e| e == "zip"),
        "expected zip archive"
    );
    ensure!(mod_zip.is_file());
    ensure!(!target.is_file());

    let (mod_name, files) = read_mod_maps(BufReader::new(File::open(mod_zip)?))?;

    for (folder, name, map_bin) in &files {
        let boilerplate = match boilerplate_map(&mod_name, folder, name, map_bin) {
            Ok(boilerplate) => boilerplate,
            Err(e) => {
                eprintln!("failed to setup boilerplate for {folder}/{name}: {e}");
                continue;
            }
        };

        let folder_path = target.join(folder);
        std::fs::create_dir_all(&folder_path)?;
        std::fs::write(folder_path.join(name).with_extension("tas"), boilerplate)?;
    }

    Ok(())
}

fn intro_type_nocontrol(intro_type: &str) -> Result<Option<u32>> {
    Ok(match intro_type {
        "Respawn" => Some(36),
        "WalkInRight" => None,
        "WalkInLeft" => None,
        "Jump" => None,
        "WakeUp" => Some(190),
        "Fall" => None,
        "TempleMirrorVoid" => return Err(anyhow!("missing wakeup time for TempleMirrorVoid")),
        "ThinkForABit" => Some(98),
        "None" => Some(0),
        _ => return Err(anyhow!("unknown intro animation: {intro_type}")),
    })
}

fn boilerplate_map(mod_name: &str, folder: &str, name: &str, map_bin: &[u8]) -> Result<String> {
    let map_raw = celesteloader::map::decode::decode_map(map_bin)?;
    let map = celesteloader::map::load_map_from_element(&map_raw)?;

    let meta = map_raw.child_with_name("meta")?;
    let intro_type = meta.get_attr::<&str>("IntroType")?;

    let mode = meta.child_with_name("mode")?;

    let start_level = mode.get_attr::<&str>("StartLevel").unwrap_or_else(|_| {
        let filler_bound_left = map
            .fillers
            .iter()
            .map(|filler| filler.position.0)
            .min()
            .unwrap_or(i32::MAX);
        let filler_bound_bottom = map
            .fillers
            .iter()
            .map(|filler| filler.position.1)
            .max()
            .unwrap_or(i32::MIN);

        let bounds_left = map
            .rooms
            .iter()
            .map(|room| room.bounds.position.x)
            .min()
            .unwrap()
            .min(filler_bound_left)
            - 64;
        let bounds_bottom = map
            .rooms
            .iter()
            .map(|room| room.bounds.position.y)
            .max()
            .unwrap()
            .max(filler_bound_bottom)
            + 64;

        let room_closest_to_start = map
            .rooms
            .iter()
            .filter(|room| room.entities.iter().any(|entity| entity.name == "player"))
            .min_by_key(|room| {
                (room.bounds.position.x - bounds_left).pow(2)
                    + (room.bounds.position.y - bounds_bottom).pow(2)
            })
            .unwrap();
        &room_closest_to_start.name
    });

    let intro_len = intro_type_nocontrol(intro_type)?;

    let wakeup = match intro_len {
        Some(0) => "".to_owned(),
        Some(len) => format!("\n{: >4}", len),
        None => "\n#TODO: replace with correct amount of intro animation frames\n   0".to_owned(),
    };

    let inputs = format!(
        r#"RecordCount:
console load {mod_name}/{folder}/{name}
   1

#Start{}

#lvl_{}
   1,J

ChapterTime:
"#,
        wakeup, start_level,
    );
    Ok(inputs)
}

fn read_mod_maps<R: Read + Seek>(
    reader: R,
) -> Result<(String, Vec<(String, String, Vec<u8>)>), anyhow::Error> {
    let mut zip = ZipArchive::new(reader)?;
    let mut map_name = None;
    let mut files = Vec::new();

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;

        if !file.is_file() {
            continue;
        }

        let Some((map, folder, name)) = file
            .name()
            .strip_prefix("Maps/")
            .and_then(|name| split_twice(name, '/'))
        else {
            continue;
        };
        match map_name.as_deref() {
            None => map_name = Some(map.to_owned()),
            Some(map_name) => {
                anyhow::ensure!(map_name == map, "expected {}, found {}", map_name, folder)
            }
        }
        let Some(name) = name.strip_suffix(".bin") else {
            continue;
        };

        let folder = folder.to_owned();
        let name = name.to_owned();

        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        files.push((folder, name, bytes));
    }

    let map_name = map_name.ok_or_else(|| anyhow!("no maps found in zip"))?;

    Ok((map_name, files))
}

fn split_twice(s: &str, delim: char) -> Option<(&str, &str, &str)> {
    let (a, b) = s.split_once(delim)?;
    let (b, c) = b.split_once(delim)?;
    Some((a, b, c))
}
