use anyhow::Result;
use celesteloader::{map::Map, CelesteInstallation};

fn main() -> Result<()> {
    let celeste = CelesteInstallation::detect()?;
    celeste.mods_with(|zip_name, mut archive| {
        let maps = archive.list_maps();
        let _maps = maps
            .into_iter()
            .map(|map_path| -> Result<_> {
                println!("{zip_name} {}", map_path.rsplit_once('/').unwrap().1);
                let data = archive.read_file(&map_path)?;
                let map = Map::parse(&data)?;
                Ok(map)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(())
    })?;
    Ok(())
}
