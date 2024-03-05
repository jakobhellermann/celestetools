use anyhow::{ensure, Context, Result};
use image::GenericImageView;
use map_annotate::{cct_physics_inspector::PhysicsInspector, Annotate, MapBounds};

fn main() -> Result<()> {
    annotate("mausoleum.png", "out.png")
}

fn annotate(map: &str, out: &str) -> Result<()> {
    let font_data: &[u8] = include_bytes!("../../DejaVuSans.ttf");
    let _font = rusttype::Font::try_from_bytes(font_data).unwrap();

    let installation = cmaploader::celeste_installations()?;
    let installation = installation
        .get(0)
        .context("could not find celeste installation")?;

    let map = image::io::Reader::open(map)?.decode()?;
    let map_dimensions = map.dimensions();

    let physics_inspector = PhysicsInspector::new(&installation);
    let recent_recordings = physics_inspector.recent_recordings()?;

    let mut map_bounds: Option<MapBounds> = None;

    let mut matching_logs = Vec::new();

    for (i, layout) in recent_recordings {
        let bounds = layout.bounds();
        if bounds.dimensions() != map_dimensions {
            continue;
        }

        match &map_bounds {
            Some(map_bounds) => {
                ensure!(
                    *map_bounds == bounds,
                    "CCT recording {i} ({}) has different map bounds: {bounds} != {map_bounds}",
                    layout.chapter_name,
                );
            }
            _ => map_bounds = Some(bounds),
        };

        eprintln!(
            "CCT recording {} in map `{}` matches image dimension...",
            i, &layout.chapter_name
        );

        matching_logs.push(i);
    }
    let map_bounds = map_bounds.context("no recording matches the dimensions of the map image")?;
    let mut annotate = Annotate::new(map, map_bounds);

    for i in matching_logs {
        annotate.annotate_cct_recording2(&physics_inspector, i)?;
    }

    annotate.save(out)?;

    /*Annotate::map(
        "mausoleum.png",
        Anchor::BottomLeft {
            room_pos: (-128 * 8, 36 * 8),
            room_height: 23 * 8,
        },
    )?
    .save("out.png")?;*/

    /*Annotate::map(
        "testing/annotate/map.png",
        Anchor::TopLeft {
            room_pos: (87 * 8, -84 * 8),
        },
    )?
    .annotate_entries("testing/annotate/maps.csv")?
    .annotate_recent_cct_recordings(|chapter_name| chapter_name == "Flowing Gallery")?
    .save("out.png")?;

    Annotate::map(
        "testing/annotate/1a.png",
        Anchor::BottomLeft {
            room_pos: (0, 0),
            room_height: 23 * 8,
        },
    )?
    .annotate_recent_cct_recordings(|chapter_name| chapter_name == "Forsaken City")?
    .save("out.png")?;*/

    /*Annotate::map(
        "mausoleum.png",
        Anchor::BottomLeft {
            room_pos: (-128 * 8, 36 * 8),
            room_height: 23 * 8,
        },
    )?
    .annotate_recent_cct_recordings(|chapter_name| chapter_name == "The Mausoleum")?
    .save("out.png")?;*/

    Ok(())
}
