use anyhow::Result;

fn main() -> Result<()> {
    for file in std::env::args().skip(1) {
        println!("-- {file} --");
        let contents = std::fs::read(&file)?;
        // let map = celesteloader::load_map(&contents).unwrap();
        let map = celesteloader::map::decode::decode_map(&contents).unwrap();

        println!("{:#?}", map);
    }

    Ok(())
}
