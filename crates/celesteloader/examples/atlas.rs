use anyhow::Result;

fn main() -> Result<()> {
    let celeste = celesteloader::celeste_installation()?;
    /*for atlas in celeste.list_atlases()? {
        for sprite in &atlas.sprites {
            // dbg!(&sprite.path);
        }
    }*/

    let gameplay_atlas = celeste
        .read_atlas_meta("Gameplay")?
        .into_iter()
        .next()
        .unwrap();
    for sprite in &gameplay_atlas.sprites {
        dbg!(&sprite.path);
        /*let image = celeste.decode_atlas_image(&atlas)?;
        image.save_with_format(atlas.data.with_extension("png"), image::ImageFormat::Png)?;*/
    }

    Ok(())
}
