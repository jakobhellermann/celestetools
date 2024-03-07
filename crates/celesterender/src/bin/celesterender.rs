use std::time::Instant;

use anyhow::Result;
use celesterender::Layer;

fn main() -> Result<()> {
    fastrand::seed(0);

    let celeste = celesteloader::celeste_installation()?;
    let map = celeste.vanilla_map("9-Core")?;

    let start = Instant::now();
    let pixmap = celesterender::render(&celeste, &map, Layer::ALL)?;
    let duration = start.elapsed();
    println!("Took {:?}", duration);

    pixmap.save_png("out.png")?;

    Ok(())
}
