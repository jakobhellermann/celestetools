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

        let room_pos = (room.get_attr_int("x")?, room.get_attr_int("y")?);
        let room_name = room.get_attr::<&str>("name")?;

        for trigger in &triggers.children {
            if trigger.name == "CollabUtils2/ChapterPanelTrigger" {
                let trigger_pos = (trigger.get_attr_int("x")?, trigger.get_attr_int("y")?);
                let map = trigger.get_attr::<&str>("map")?;

                let name = dialog
                    .get(map)
                    .with_context(|| format!("getting name of {map} from dialog"))?;

                let room_size = (room.get_attr_int("width")?, room.get_attr_int("height")?);
                if trigger_pos.0 > room_size.0 || trigger_pos.1 > room_size.1 {
                    continue;
                }

                maps.push((
                    name,
                    room_pos.0 + trigger_pos.0,
                    room_pos.1 + trigger_pos.1,
                    room_name,
                ));
            }
        }
    }

    maps.sort_by_key(|&(name, ..)| name);

    for (name, x, y, room_name) in maps {
        println!("{name},{x},{y} in {room_name}");
    }

    Ok(())
}
