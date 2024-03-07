use std::{collections::HashMap, fs::File, io::BufReader};

use annotate_celeste_map::{cct_physics_inspector::PhysicsInspector, Annotate};
use anyhow::{anyhow, bail, Result};
use celesteloader::{archive::ModArchive, map::Map, CelesteInstallation};
use celesterender::{
    asset::{AssetDb, ModLookup},
    CelesteRenderData, Layer,
};

fn main() -> Result<()> {
    let celeste = CelesteInstallation::detect()?;
    let physics_inspector = PhysicsInspector::new(&celeste);

    let term = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("expected argument for CCT filter"))?
        .to_lowercase();

    let sids: HashMap<String, Vec<u32>> = physics_inspector
        .recent_recordings()?
        .into_iter()
        .filter(|(_, layout)| layout.chapter_name.to_lowercase().contains(&term))
        .fold(HashMap::new(), |mut acc, (i, layout)| {
            acc.entry(layout.sid.unwrap()).or_default().push(i);
            acc
        });

    let (sid, recordings) = match sids.len() {
        0 => bail!("None found"),
        1 => sids.into_iter().next().unwrap(),
        _ => bail!(
            "found different matching CCT recordings: ({:?})",
            sids.keys().collect::<Vec<_>>()
        ),
    };

    let (map, fgtiles, bgtiles) = celeste
        .find_mod_with(|_, mut archive| {
            let map = archive.try_read_file(&format!("Maps/{sid}.bin"))?;
            let map = map.map(|data| Map::parse(&data)).transpose()?;

            map.map(|map| -> Result<_> {
                let fgtiles = map
                    .meta
                    .foreground_tiles
                    .as_ref()
                    .map(|path| archive.read_file_string(path))
                    .transpose()?;
                let bgtiles = map
                    .meta
                    .background_tiles
                    .as_ref()
                    .map(|path| archive.read_file_string(path))
                    .transpose()?;

                Ok((map, fgtiles, bgtiles))
            })
            .transpose()
        })?
        .unwrap();

    let fgtiles = match fgtiles {
        Some(fgtiles) => fgtiles,
        None => celeste.read_to_string("Content/Graphics/ForegroundTiles.xml")?,
    };
    let bgtiles = match bgtiles {
        Some(bgtiles) => bgtiles,
        None => celeste.read_to_string("Content/Graphics/BackgroundTiles.xml")?,
    };

    let mods = celeste
        .list_mod_zips()?
        .into_iter()
        .map(File::open)
        .collect::<Result<Vec<_>, std::io::Error>>()?;
    let mut mods = mods
        .iter()
        .map(|data| ModArchive::new(BufReader::new(data)))
        .collect::<Result<Vec<_>, _>>()?;
    let mut asset_db = AssetDb::new(ModLookup::new(mods.as_mut_slice()));

    let mut celeste_render_data = CelesteRenderData::base(&celeste)?;
    celeste_render_data.load_tilesets(&fgtiles, &bgtiles)?;

    let image = celesterender::render_with(&celeste_render_data, &mut asset_db, &map, Layer::ALL)?;
    let image = image::RgbaImage::from_vec(image.width(), image.height(), image.take()).unwrap();

    let mut annotate = Annotate::new(image.into(), map.bounds().into());
    for i in recordings {
        annotate.annotate_cct_recording(&physics_inspector, i)?;
    }
    annotate.save("out.png")?;

    Ok(())
}
