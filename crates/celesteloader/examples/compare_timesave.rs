use std::{collections::HashMap, ops::Range};

use anyhow::Result;
use celesteloader::{cct_physics_inspector::PhysicsInspector, map::Map, CelesteInstallation};

fn main() -> Result<()> {
    let celeste = CelesteInstallation::detect()?;
    let pi = celeste.physics_inspector();

    let mut mods = celeste.all_mods()?;

    let mut recordings = HashMap::<_, Vec<u32>>::new();
    for (i, layout) in pi.recent_recordings()? {
        let Some(map_bin) = layout.map_bin else {
            eprintln!("old cct, skipping");
            continue;
        };

        recordings.entry(map_bin).or_default().push(i);
    }

    for (map_bin, recordings) in recordings {
        let mut found = None;
        for archive in &mut mods {
            if let Some(f) = archive.try_read_file(&format!("Maps/{map_bin}.bin"))? {
                let map = Map::parse(&f)?;
                found = Some((archive, map));
            }
        }

        let Some((archive, map)) = found else {
            eprintln!("map {map_bin} not found");
            continue;
        };

        let dialog = archive.get_dialog("English")?;
        let map_name = dialog.get(&map_bin).unwrap_or(&map_bin);

        let mut recordings = recordings
            .iter()
            .filter_map(|&recording| fun_name(&pi, recording, &map).transpose())
            .collect::<Result<Vec<_>>>()?;

        if recordings.len() != 2 {
            /*eprintln!(
                "> {map_name}: found {} recordings instead of 2, skipping",
                recordings.len()
            );*/
            continue;
        }
        let a = recordings.pop().unwrap();
        let b = recordings.pop().unwrap();

        let ((old_time_total, old_rooms), (new_time_total, new_rooms)) =
            if a.0 > b.0 { (a, b) } else { (b, a) };

        println!(
            "-{}f {} {} -> {}",
            old_time_total - new_time_total,
            map_name,
            frames_to_finaltime(old_time_total),
            frames_to_finaltime(new_time_total)
        );

        for (room_idx, room_name, new) in new_rooms {
            let (_, _, old) = old_rooms
                .iter()
                .find(|&&(i, n, ..)| i == room_idx && n == room_name)
                .unwrap();
            let old_time = old.len();
            let new_time = new.len();

            let room_diff = new_time as i32 - old_time as i32;

            if room_diff != 0 {
                let room_label = if room_idx == 0 {
                    room_name.to_owned()
                } else {
                    format!("{room_name} ({room_idx})")
                };
                println!(
                    "{}{}f [{room_label}]:",
                    if room_diff.is_positive() { "+" } else { "" },
                    room_diff
                );

                // println!("new: {:?}, old: {:?}", new, old)
            }
        }
    }

    Ok(())
}

fn fun_name<'a>(
    pi: &PhysicsInspector,
    position_log: u32,
    map: &'a Map,
) -> Result<Option<(u32, Vec<(usize, &'a str, Range<u32>)>)>, anyhow::Error> {
    let mut first_position = None;
    let mut all_same = true;

    let mut last_frame = 0;

    let mut room_entries: Vec<(usize, &str, Range<u32>)> = Vec::new();
    for item in pi.position_log(position_log)? {
        let item = item?;
        let frame = item.frame_rta - 1;

        let first_frame_in_room = item.flags.contains("FirstFrameInRoom");

        let first_position = *first_position.get_or_insert((item.x, item.y));
        if first_position.0 != item.x || first_position.1 != item.y {
            all_same = false;
        }

        let room_name = &map
            .room_at(
                item.x + item.speed_x.signum() * 8.,
                item.y + item.speed_y.signum() * 12.,
            )
            .unwrap()
            .name;

        let flush = first_frame_in_room;

        if flush {
            if let Some(prev) = room_entries.last_mut() {
                prev.2.end = frame;
            }
            room_entries.push((0, room_name.as_str(), frame..frame))
        }

        last_frame = frame;
    }

    Ok((!all_same).then_some((last_frame, room_entries)))
}

fn frames_to_finaltime(frames: u32) -> String {
    let ms = frames * 17;
    let s = ms / 1000;
    let min = s / 60;

    format!("{}:{:0>2}.{:0>3}({frames})", min, s % 60, ms % 1000)
}
