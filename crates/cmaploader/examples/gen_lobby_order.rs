use anyhow::{Context, Result};
fn main() -> Result<()> {
    let dialog = std::fs::read_to_string("testing/VanillaContest2023/Dialog/English.txt")?;
    let dialog = cmaploader::dialog::Dialog::from_txt(&dialog);

    let contents =
        std::fs::read("testing/VanillaContest2023/Maps/VanillaContest2023/0-Lobbies/Lobby.bin")?;
    let map = cmaploader::decode::decode_map(&contents)?;

    let rooms = map.child_with_name("levels")?;

    let mut maps = Vec::new();

    let mut default_spawn = None;
    let mut first_spawn = None;
    let mut heart_door = None;

    for room in &rooms.children {
        let room_pos = (room.get_attr_int("x")?, room.get_attr_int("y")?);
        let room_name = room.get_attr::<&str>("name")?;

        let Some(triggers) = room.find_child_with_name("triggers") else {
            continue;
        };

        let entities = room.child_with_name("entities")?;

        for entity in &entities.children {
            match entity.name {
                "CollabUtils2/MiniHeartDoor" => {
                    let pos = (entity.get_attr_int("x")?, entity.get_attr_int("y")?);
                    let size = (
                        entity.get_attr_int("height")?,
                        entity.get_attr_int("width")?,
                    );

                    heart_door =
                        Some((room_pos.0 + pos.0 + size.0 / 2, room_pos.1 + pos.1 + size.1));
                }
                "player" => {
                    let entity_pos = (entity.get_attr_int("x")?, entity.get_attr_int("y")?);
                    let pos = (room_pos.0 + entity_pos.0, room_pos.1 + entity_pos.1);

                    if first_spawn.is_none() {
                        first_spawn = Some(pos);
                    }

                    let is_default_spawn = entity
                        .attributes
                        .get("isDefaultSpawn")
                        .and_then(|a| a.get::<bool>())
                        .unwrap_or(false);

                    if is_default_spawn {
                        default_spawn = Some(pos);
                    }
                }
                _ => {}
            }
        }

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

    if let Some((x, y)) = default_spawn.or(first_spawn) {
        println!("0,\"Start\",{x},{y}");
    }
    let mut idx = 1;
    for (_room, maps) in maps {
        for (name, x, y) in maps {
            println!("{idx},\"{}\",{x},{y}", name);
            idx += 1;
        }
    }

    if let Some((x, y)) = heart_door {
        println!("{idx},\"Heartside\",{x},{y}");
    }

    Ok(())
}
