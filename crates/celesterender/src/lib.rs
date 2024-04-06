#![allow(clippy::wildcard_in_or_patterns, clippy::too_many_arguments)]

use anyhow::Result;

pub mod asset;
mod rendering;

use asset::{AssetDb, LookupAsset};
use celesteloader::{map::Map, CelesteInstallation};
pub use png::Compression;
pub use rendering::{
    render, CelesteRenderData, Layer, MapTileset, RenderMapSettings, RenderResult,
};

pub fn render_map_bin(
    celeste: &CelesteInstallation,
    render_data: &mut CelesteRenderData,
    asset_db: &mut AssetDb<impl LookupAsset>,
    map_bin: &str,
    settings: RenderMapSettings<'_>,
) -> Result<(RenderResult, Map)> {
    let (map, mut archive) = celeste.find_map_by_map_bin(map_bin)?;

    if let Some(archive) = &mut archive {
        render_data.load_map_tileset(celeste, archive, &map)?;
    } else {
        render_data.map_tileset = MapTileset::vanilla(celeste)?;
    }

    let image = render(render_data, asset_db, &map, settings)?;

    Ok((image, map))
}
