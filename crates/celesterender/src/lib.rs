use anyhow::{anyhow, Context, Result};

pub mod asset;
mod rendering;

use asset::{AssetDb, LookupAsset};
use celesteloader::{map::Map, CelesteInstallation};
pub use rendering::{
    render, render_with, CelesteRenderData, Layer, MapTileset, RenderMapSettings, RenderResult,
};

pub fn render_map_bin(
    celeste: &CelesteInstallation,
    render_data: &mut CelesteRenderData,
    asset_db: &mut AssetDb<impl LookupAsset>,
    map_bin: &str,
    settings: RenderMapSettings<'_>,
) -> Result<(RenderResult, Map)> {
    let (map, fgtiles, bgtiles) = if let Some(vanilla_sid) = map_bin.strip_prefix("Celeste/") {
        let map = celeste.vanilla_map(&vanilla_sid)?;
        (map, None, None)
    } else {
        celeste
            .find_mod_with(|_, mut archive| {
                let map = archive.try_read_file(&format!("Maps/{map_bin}.bin"))?;
                let map = map.map(|data| Map::parse(&data)).transpose()?;

                map.map(|map| -> Result<_> {
                    let (fgtiles, bgtiles) = archive.map_fgtiles_bgtiles(&map)?;
                    Ok((map, fgtiles, bgtiles))
                })
                .transpose()
            })?
            .with_context(|| anyhow!("could not find map .bin for {map_bin}"))?
    };

    let fgtiles = match fgtiles {
        Some(fgtiles) => fgtiles,
        None => celeste.read_to_string("Content/Graphics/ForegroundTiles.xml")?,
    };
    let bgtiles = match bgtiles {
        Some(bgtiles) => bgtiles,
        None => celeste.read_to_string("Content/Graphics/BackgroundTiles.xml")?,
    };

    render_data.map_tileset = MapTileset::parse(&fgtiles, &bgtiles)?;

    let image = render_with(render_data, asset_db, &map, settings)?;

    Ok((image, map))
}
