use anyhow::Result;
use celesteloader::{map::Map, CelesteInstallation};

fn main() -> Result<()> {
    let celeste = CelesteInstallation::detect()?;

    let entity_name = "floatySpaceBlock";

    let mut sj = celeste.read_mod("SpringCollab2020")?;
    let dialog = sj.get_dialog("English")?;

    let mut results = Vec::new();

    let maps = sj.list_maps();
    for map_path in maps {
        let data = sj.read_file(&map_path)?;
        let map = match Map::parse(&data) {
            Ok(map) => map,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };

        let set = map_path
            .trim_start_matches("Maps/")
            .trim_end_matches(".bin");
        let name = dialog.get(set).unwrap();

        let lobby = set.split('/').nth(1).unwrap().rsplit_once('-').unwrap().1;

        let n = map
            .rooms
            .iter()
            .flat_map(|room| {
                room.entities
                    .iter()
                    .filter(|entity| entity.name == entity_name)
            })
            .count();

        if n > 0 {
            results.push((n, name.to_owned(), lobby.to_owned()));
        }
    }

    results.sort_by_key(|x| std::cmp::Reverse(x.0));

    for (n, map, lobby) in results {
        println!("{n}: ({lobby}) {map}");
    }

    Ok(())
}
