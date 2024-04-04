use std::{fmt::Write, ops::Range};

use super::PhysicsInspector;
use crate::map::Map;
use anyhow::Result;

pub fn compare_timesave(
    physics_inspector: &PhysicsInspector,
    map: &Map,
    map_name: &str,
    recordings: (u32, u32),
) -> Result<String> {
    let rec_0 = sync_segments(map, physics_inspector, recordings.0)?;
    let rec_1 = sync_segments(map, physics_inspector, recordings.1)?;

    let (slow, fast) = if rec_0.0 > rec_1.0 {
        (rec_0, rec_1)
    } else {
        (rec_1, rec_0)
    };

    let rendered = render_sync_segment_improvement(map_name, fast, slow);
    Ok(rendered)
}

fn render_sync_segment_improvement(
    map_name: &str,
    (fast_time, new_segments): (u32, Vec<SyncSegment<'_>>),
    (slow_time, old_segments): (u32, Vec<SyncSegment<'_>>),
) -> String {
    let mut s = String::new();

    let time_diff = fast_time as i32 - slow_time as i32;
    let _ = writeln!(
        &mut s,
        "{}{}f {} {} -> {}",
        if time_diff > 0 { "+" } else { "" },
        time_diff,
        map_name,
        frames_to_finaltime(slow_time),
        frames_to_finaltime(fast_time)
    );

    for new_segment in new_segments {
        let room_label = new_segment.name();

        let old_segment = old_segments.iter().find(|&old_segment| {
            old_segment.room_idx == new_segment.room_idx
                && old_segment.room_name == new_segment.room_name
        });
        let Some(old_segment) = old_segment else {
            let _ = writeln!(&mut s, "-?f [{room_label}]:",);
            continue;
        };

        let room_diff = new_segment.time.len() as i32 - old_segment.time.len() as i32;
        if room_diff != 0 {
            let _ = writeln!(
                &mut s,
                "{}{}f [{room_label}]:",
                if room_diff.is_positive() { "+" } else { "" },
                room_diff
            );
        }
    }

    s
}

#[derive(Debug)]
struct SyncSegment<'a> {
    room_idx: u32,
    room_name: &'a str,

    time: Range<u32>,
}

fn sync_segments<'a>(
    map: &'a Map,
    physics_inspector: &PhysicsInspector,
    position_log: u32,
) -> Result<(u32, Vec<SyncSegment<'a>>), anyhow::Error> {
    let mut first_position = None;
    let mut _all_same = true;

    let mut last_frame = 0;

    let mut room_entries: Vec<SyncSegment> = Vec::new();
    for item in physics_inspector.position_log(position_log)? {
        let item = item?;
        let frame = item.frame_rta - 1;

        let first_frame_in_room = item.flags.contains("FirstFrameInRoom");

        let first_position = *first_position.get_or_insert((item.x, item.y));
        if first_position.0 != item.x || first_position.1 != item.y {
            _all_same = false;
        }

        let room_name = map
            .room_at(
                item.x + item.speed_x.signum() * 8.,
                item.y + item.speed_y.signum() * 12.,
            )
            .map(|room| room.name.as_str())
            .unwrap_or("?");

        if first_frame_in_room {
            if let Some(prev) = room_entries.last_mut() {
                prev.time.end = frame;
            }
            room_entries.push(SyncSegment {
                room_idx: 0,
                room_name,
                time: frame..frame,
            });
        }

        last_frame = frame;
    }

    if let Some(last) = room_entries.last_mut() {
        last.time.end = last_frame + 1;
    }

    assert_eq!(
        last_frame + 1,
        room_entries
            .iter()
            .map(|entry| entry.time.len() as u32)
            .sum::<u32>()
    );

    // Ok((!all_same).then_some((last_frame + 1, room_entries)))
    Ok((last_frame + 1, room_entries))
}

impl SyncSegment<'_> {
    fn name(&self) -> String {
        if self.room_idx == 0 {
            self.room_name.to_owned()
        } else {
            format!("{} ({})", self.room_name, self.room_idx)
        }
    }
}

fn frames_to_finaltime(frames: u32) -> String {
    let ms = frames * 17;
    let s = ms / 1000;
    let min = s / 60;

    format!("{}:{:0>2}.{:0>3}({frames})", min, s % 60, ms % 1000)
}
