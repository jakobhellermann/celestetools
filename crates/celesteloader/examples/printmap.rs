use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use celesteloader::{archive::ModArchive, CelesteInstallation};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);

    while let Some(file) = args.next() {
        let map = if file.ends_with(".zip") {
            let zip = if Path::new(&file).exists() {
                PathBuf::from(file.clone())
            } else {
                let celeste = CelesteInstallation::detect()?;
                celeste.path.join("Mods").join(&file)
            };

            let map_name = args.next().context("map name expected as second arg")?;

            let mut zip = ModArchive::new(BufReader::new(File::open(zip)?))?;
            let map_path = zip
                .list_files()
                .find(|map| map.ends_with(".bin") && map.contains(&map_name))
                .unwrap()
                .to_owned();
            zip.read_file(&map_path)?
        } else {
            std::fs::read(&file)?
        };

        println!("-- {} --", file);
        // let map = celesteloader::map::load_map(&map).unwrap();
        let map = celesteloader::map::decode::decode_map(&map).unwrap();

        println!("{:#?}", map);
    }

    Ok(())
}
