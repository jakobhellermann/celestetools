use std::{fs::File, io::BufReader, path::PathBuf};

use anyhow::Result;
use celesteloader::{archive::ModArchive, map::Map, CelesteInstallation};

fn main() -> Result<()> {
    let celeste = CelesteInstallation::detect()?;

    let mod_dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| celeste.path.join("Mods"));

    celesteloader::utils::list_dir_extension::<_, anyhow::Error>(&mod_dir, "zip", |path| {
        let mut archive = ModArchive::new(BufReader::new(File::open(path)?))?;
        let maps = archive.list_maps();
        let _maps = maps
            .into_iter()
            .map(|map_path| {
                let data = match archive.read_file(&map_path) {
                    Ok(data) => data,
                    Err(e) => return (map_path, Err(anyhow::Error::from(e))),
                };
                let map = Map::parse(&data).map_err(From::from);
                (map_path, map)
            })
            .collect::<Vec<_>>();

        for (map_path, map) in _maps {
            let filename = map_path.rsplit_once('/').unwrap().1;

            if let Err(e) = map {
                eprintln!("{filename} Failed to read map: {e}");
            } else {
                // println!("{filename} {}", map_path.rsplit_once('/').unwrap().1);
            }
        }

        Ok(())
    })?;

    Ok(())
}
