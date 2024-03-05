use anyhow::Result;
use map_annotate::{Anchor, Annotate};

fn main() -> Result<()> {
    Annotate::map(
        "testing/annotate/map.png",
        Anchor::TopLeft {
            room_pos: (87 * 8, -84 * 8),
        },
    )?
    .annotate_entries("testing/annotate/maps.csv")?
    .annotate_recent_cct_recordings(|chapter_name| chapter_name == "Flowing Gallery")?
    .save("out.png")?;

    /*Annotate::map(
        "testing/annotate/1a.png",
        Anchor::BottomLeft {
            room_pos: (0, 0),
            room_height: 23 * 8,
        },
    )?
    .annotate_recent_cct_recordings(|chapter_name| chapter_name == "Forsaken City")?
    .save("out.png")?;*/

    Ok(())
}
