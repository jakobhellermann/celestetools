use anyhow::{Context, Result};

fn main() -> Result<()> {
    let file = std::env::args().nth(1).context("no file passed")?;

    let contents = std::fs::read(&file)?;
    let map = cmaploader::decode::decode_map(&contents)?;

    let rooms = map.child_with_name("levels")?;

    let mut maps = Vec::new();

    for room in &rooms.children {
        let Some(triggers) = room.find_child_with_name("triggers") else {
            continue;
        };
        for trigger in &triggers.children {
            if trigger.name == "CollabUtils2/ChapterPanelTrigger" {
                let x = trigger.get_attr_int("x")?;
                let y = trigger.get_attr_int("y")?;
                let map = trigger.get_attr::<&str>("map")?;

                maps.push((map, x, y));
            }
        }
    }

    dbg!(maps);

    Ok(())
}
