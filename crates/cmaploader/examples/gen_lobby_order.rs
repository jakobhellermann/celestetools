use anyhow::{Context, Result};
fn main() -> Result<()> {
    let dialog = std::fs::read_to_string("testing/VanillaContest2023/Dialog/English.txt")?;
    let dialog = cmaploader::dialog::Dialog::from_txt(&dialog);

    let contents =
        std::fs::read("testing/VanillaContest2023/Maps/VanillaContest2023/0-Lobbies/Lobby.bin")?;
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

                let name = dialog
                    .get(map)
                    .with_context(|| format!("getting name of {map} from dialog"))?;

                maps.push((name, x, y));
            }
        }
    }

    for (name, x, y) in maps {
        // println!("{x},{y},{name}")
        println!("{}", name);
    }

    Ok(())
}
