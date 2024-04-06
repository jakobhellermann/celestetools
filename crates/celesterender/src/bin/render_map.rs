use std::fmt::Write;

use anyhow::{Context, Result};
use celesteloader::CelesteInstallation;
use celesterender::{
    asset::{AssetDb, ModLookup},
    CelesteRenderData, RenderMapSettings,
};

fn main() -> Result<()> {
    let celeste = CelesteInstallation::detect()?;

    let mod_name = "strawberryjam";
    let map_name = "fragments";

    let mut archive = celeste
        .find_mod_with(|name, archive| {
            Ok(name
                .to_ascii_lowercase()
                .contains(mod_name)
                .then_some(archive))
        })?
        .with_context(|| format!("'{mod_name}' not found"))?;

    let dialog = archive.get_dialog("English").context("no dialog")?;
    let map_path = archive
        .list_map_files()
        .into_iter()
        .find(|map| {
            let map_bin = map.trim_start_matches("Maps/").trim_end_matches(".bin");
            let name = dialog.get(map_bin).unwrap();
            name.to_lowercase().contains(map_name)
        })
        .with_context(|| format!("'{map_name}' not found in '{mod_name}'"))?;

    let map = archive.read_map(&map_path)?;

    let render_data = CelesteRenderData::for_map(&celeste, &mut archive, &map)?;
    let asset_db = ModLookup::all_mods(&celeste)?;
    let mut result = celesterender::render(
        &render_data,
        &mut AssetDb::new(asset_db),
        &map,
        RenderMapSettings::default(),
    )?;

    result.save_png("out/saved.png", png::Compression::Default)?;

    if !result.unknown_entities.is_empty() {
        let mut unknown = result.unknown_entities.iter().collect::<Vec<_>>();
        unknown.sort_by_key(|&(_, n)| std::cmp::Reverse(n));

        eprintln!(
            "Found {:2} unknown entities: ({} ...)",
            unknown.len(),
            unknown
                .iter()
                .take(5)
                .fold(String::new(), |mut acc, (name, num)| {
                    let _ = write!(&mut acc, "{num} {name} ");
                    acc
                })
        );
    }

    Ok(())
}
