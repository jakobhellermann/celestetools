use std::{path::PathBuf, time::Instant};

use anyhow::Result;
use celesterender::Layer;

fn main() -> Result<()> {
    fastrand::seed(0);

    let celeste = celesteloader::celeste_installation()?;

    let out = PathBuf::from("out");
    std::fs::create_dir_all(&out)?;

    for map in celeste.vanilla_maps()? {
        let start = Instant::now();
        let pixmap = celesterender::render(&celeste, &map, Layer::ALL)?;
        let duration = start.elapsed();
        println!(
            "Took {:>4.2?}ms to render {}",
            duration.as_millis(),
            map.package
        );

        pixmap.save_png(out.join(&map.package).with_extension("png"))?;
    }

    Ok(())
}
