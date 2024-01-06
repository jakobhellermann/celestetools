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

        let mut room_maps = Vec::new();

        for trigger in &triggers.children {
            if trigger.name == "CollabUtils2/ChapterPanelTrigger" {
                let trigger_pos = (trigger.get_attr_int("x")?, trigger.get_attr_int("y")?);
                let trigger_size = (
                    trigger.get_attr_int("width")?,
                    trigger.get_attr_int("height")?,
                );
                let map = trigger.get_attr::<&str>("map")?;

                let name = dialog
                    .get(map)
                    .with_context(|| format!("getting name of {map} from dialog"))?;

                let room_size = (room.get_attr_int("width")?, room.get_attr_int("height")?);
                if trigger_pos.0 > room_size.0 || trigger_pos.1 > room_size.1 {
                    continue;
                }

                let x = room_pos.0 + trigger_pos.0 + trigger_size.0 / 2;
                let y = room_pos.1 + trigger_pos.1 + trigger_size.1;
                room_maps.push((name, x, y));
            }
        }

        if !room_maps.is_empty() {
            maps.push((room_name.to_owned(), room_maps))
        }
    }

    maps.iter_mut()
        .for_each(|(_, maps)| maps.sort_by_key(|&(_, x, y)| (y, x)));

    let mut idx = 0;

    for (room, maps) in maps {
        for (name, x, y) in maps {
            println!("{idx},\"{}\",{x},{y}", name);
            idx += 1;
        }
    }

    Ok(())
}
