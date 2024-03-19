#![allow(dead_code)]

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Instant;
use std::{borrow::Cow, sync::Arc};

use anyhow::{Context, Result};
use celesteloader::{archive::ModArchive, map::Map, CelesteInstallation};
use celesterender::{
    asset::{AssetDb, LookupAsset, ModLookup},
    CelesteRenderData, MapTileset, RenderMapSettings,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn render_map<L: LookupAsset>(
    asset_db: &mut AssetDb<L>,
    zip: &mut ModArchive<BufReader<File>>,
    render_data: &mut CelesteRenderData,
    map_name: &str,
    vanilla_fgtiles_xml: &str,
    vanilla_bgtiles_xml: &str,
) -> Result<celesterender::RenderResult> {
    let data = zip.read_file(map_name)?;
    let map = Map::parse(&data)?;

    let (fgtiles, bgtiles) = zip.map_fgtiles_bgtiles(&map)?;

    let fgtiles = fgtiles
        .map(Cow::Owned)
        .unwrap_or_else(|| Cow::Borrowed(vanilla_fgtiles_xml));
    let bgtiles = bgtiles
        .map(Cow::Owned)
        .unwrap_or_else(|| Cow::Borrowed(vanilla_bgtiles_xml));
    render_data.map_tileset = MapTileset::parse(&fgtiles, &bgtiles)?;

    let out =
        celesterender::render_with(render_data, asset_db, &map, RenderMapSettings::default())?;

    Ok(out)
}

fn main() -> Result<()> {
    // render_modded_maps()?;
    #[cfg(feature = "tracing_chrome")]
    let _guard = {
        use tracing_subscriber::prelude::*;
        let (chrome_layer, _guard) = tracing_chrome::ChromeLayerBuilder::new()
            .include_args(true)
            .build();
        tracing_subscriber::registry().with(chrome_layer).init();
        _guard
    };

    let _celeste = CelesteInstallation::detect()?;
    render_vanilla_maps(&_celeste)?;

    Ok(())
}

fn render_modded_maps() -> Result<()> {
    let celeste = CelesteInstallation::detect()?;
    let mut asset_db = AssetDb::new(ModLookup::all_mods(&celeste)?);

    let mut render_data = CelesteRenderData::base(&celeste)?;
    let vanilla_fgtiles_xml = celeste.read_to_string("Content/Graphics/ForegroundTiles.xml")?;
    let vanilla_bgtiles_xml = celeste.read_to_string("Content/Graphics/BackgroundTiles.xml")?;

    /*let downloaded_mods: Vec<File> = Path::new("downloads")
    .read_dir()
    .unwrap()
    .map(|x| File::open(x.unwrap().path()).unwrap())
    .collect();*/
    // for m in downloaded_mods {
    // let mut zip = ModArchive::new(BufReader::new(m))?;

    celeste.read_mod("StrawberryJam2021", |mut zip| {
        let mut maps = zip.list_maps();
        maps.sort();

        let out_dir = PathBuf::from("out");
        std::fs::create_dir_all(&out_dir)?;

        for map_name in maps.iter() {
            let last_part = map_name.rsplit_once('/').unwrap().1;
            let img_path = out_dir.join(last_part).with_extension("png");

            if !map_name.contains("0-Lobbies/1-Beginner") {
                continue;
            }

            // if img_path.exists() {
            // continue;
            // }

            let res = render_map(
                &mut asset_db,
                &mut zip,
                &mut render_data,
                map_name,
                &vanilla_fgtiles_xml,
                &vanilla_bgtiles_xml,
            );
            match res {
                Err(e) => {
                    eprintln!("Error rendering {last_part}: {e}");
                }
                Ok(result) => {
                    result.image.save_png(img_path).context("saving png")?;
                    eprintln!("Successfully rendered {last_part}");
                }
            }
            // }
        }
        Ok(())
    })?;

    Ok(())
}

fn render_vanilla_maps(celeste: &CelesteInstallation) -> Result<()> {
    let out = PathBuf::from("out");
    std::fs::create_dir_all(&out)?;

    let start = Instant::now();

    let unknown_total = Arc::new(AtomicU32::new(0));

    celeste
        .vanilla_maps()?
        .par_iter()
        .try_for_each::<_, Result<_>>(|map| {
            if map.package.contains("Lost") {
                return Ok(());
            }

            let start = Instant::now();
            let result = celesterender::render(
                celeste,
                &map,
                RenderMapSettings::default().include_room(&|room| room.name.starts_with("")),
            )?;
            let duration = start.elapsed();

            result
                .image
                .save_png(out.join(&map.package).with_extension("png"))?;

            if result.unknown_entities.len() > 0 {
                let mut unknown = result.unknown_entities.iter().collect::<Vec<_>>();
                unknown.sort_by_key(|&(_, n)| std::cmp::Reverse(n));

                let x = result.unknown_entities.values().copied().sum::<u32>();
                unknown_total.fetch_add(x, Ordering::Release);

                eprintln!(
                    "Took {:4.2}ms to render {:<20} Found {:2} unknown entities: ({} ...)",
                    duration.as_millis(),
                    map.package,
                    unknown.len(),
                    unknown
                        .iter()
                        .take(3)
                        .map(|(name, num)| format!("{num} {name} "))
                        .collect::<String>()
                );
            }
            Ok(())
        })?;

    println!(
        "Total {:>4.2?}ms, {} unknown",
        start.elapsed().as_millis(),
        unknown_total.load(Ordering::Acquire)
    );

    Ok(())
}
