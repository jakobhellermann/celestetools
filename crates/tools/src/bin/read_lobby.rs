use anyhow::{Context, Result};
use celesteloader::{dialog::Dialog, map::decode::Element, CelesteInstallation};

fn main() -> Result<()> {
    let celeste = CelesteInstallation::detect()?;
    let m = "catfish";

    let mut archive = celeste
        .find_mod_with(|modname, archive| {
            Ok(modname
                .to_lowercase()
                .contains(&m.to_lowercase())
                .then_some(archive))
        })?
        .with_context(|| format!("could not find {m}"))?;

    let dialog = archive.get_dialog("English")?;
    let lobbies = archive
        .list_files()
        .filter(|name| {
            name.ends_with(".bin")
                && name
                    .rfind('/')
                    .map_or(false, |idx| name[..idx].ends_with("0-Lobbies"))
        })
        .filter(|name| name.contains("Maps"))
        .map(String::from)
        .collect::<Vec<_>>();

    for lobby in lobbies {
        let data = archive.read_file(&lobby)?;
        let map = celesteloader::map::decode::decode_map(&data)?;

        let lobby_maps = gen_lobby(map, &dialog)?;

        if lobby_maps.len() <= 1 {
            continue; // e.g. Prologue
        }

        for (i, (name, x, y)) in lobby_maps.iter().enumerate() {
            println!("{i},\"{}\",{x},{y}", name);
        }
    }

    Ok(())
}

fn gen_lobby<'a>(map: Element<'_>, dialog: &'a Dialog) -> Result<Vec<(&'a str, i32, i32)>> {
    let rooms = map.child_with_name("levels")?;

    let mut maps = Vec::new();
    let mut benches = Vec::new();

    let mut default_spawn = None;
    let mut first_spawn = None;
    let mut _heart_door = None;

    for room in &rooms.children {
        let room_pos = (room.get_attr_int("x")?, room.get_attr_int("y")?);
        let room_name = room.get_attr::<&str>("name")?;

        let Some(triggers) = room.find_child_with_name("triggers") else {
            continue;
        };

        let entities = room.child_with_name("entities")?;

        for entity in &entities.children {
            if entity.name.contains("CollabUtils2/LobbyMapWarp") {
                let pos = (entity.get_attr_int("x")?, entity.get_attr_int("y")?);
                let warp_id = entity.get_attr::<&str>("warpId")?;

                let x = room_pos.0 + pos.0;
                let y = room_pos.1 + pos.1;
                dbg!(pos);
                benches.push((warp_id, x, y));
            }
            match entity.name {
                "CollabUtils2/MiniHeartDoor" => {
                    let pos = (entity.get_attr_int("x")?, entity.get_attr_int("y")?);
                    let size = (
                        entity.get_attr_int("height")?,
                        entity.get_attr_int("width")?,
                    );

                    _heart_door =
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

                    if is_default_spawn && default_spawn.is_none() {
                        default_spawn = Some(pos);
                    }
                }
                _ => {}
            }
        }

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
                maps.push((name, room_name, x, y));
            }
        }
    }
    maps.sort_by_key(|&(_, room, x, y)| (room, y, x));
    benches.sort_by_key(|&(id, ..)| id);

    let mut results = Vec::with_capacity(maps.len() + 2);

    if let Some((x, y)) = default_spawn.or(first_spawn) {
        results.push(("Start", x, y));
    }
    for (name, _, x, y) in maps {
        results.push((name, x, y));
    }

    for (warp_id, x, y) in benches {
        let name = match warp_id.parse::<u8>() {
            Ok(warp_id) => {
                let name = (b'A' + warp_id) as char;
                format!("bench_{}", name).leak()
            }
            Err(_) => format!("bench_{}", warp_id).leak(),
        };
        results.push((name, x, y));
    }
    /*if let Some((x, y)) = heart_door {
        results.push(("Heartside", x, y));
    }*/

    Ok(results)
}
