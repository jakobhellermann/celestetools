use anyhow::Result;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);

    while let Some(file) = args.next() {
        let map = if file.ends_with(".zip") {
            let map_name = args.next().unwrap();

            celesteloader::celeste_installation()?.read_mod(&file, |mut m| {
                let map_path = m
                    .list_files()
                    .filter(|map| map.ends_with(".bin") && map.contains(&map_name))
                    .next()
                    .unwrap()
                    .to_owned();
                let contents = m.read_file(&map_path)?;
                Ok(contents)
            })?
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
