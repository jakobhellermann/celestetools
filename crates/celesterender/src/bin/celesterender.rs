#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fmt::Write;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Instant;
use std::{borrow::Cow, sync::Arc};

use anyhow::{Context, Result};
use celesteloader::{archive::ModArchive, CelesteInstallation};
use celesterender::Layer;
use celesterender::{
    asset::{AssetDb, LookupAsset, ModLookup},
    CelesteRenderData, MapTileset, RenderMapSettings,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn render_map<L: LookupAsset>(
    asset_db: &mut AssetDb<L>,
    zip: &mut ModArchive<BufReader<File>>,
    render_data: &mut CelesteRenderData,
    map: &str,
    vanilla_fgtiles_xml: &str,
    vanilla_bgtiles_xml: &str,
) -> Result<celesterender::RenderResult> {
    let map = zip.read_map(map)?;

    let (fgtiles, bgtiles) = zip.map_fgtiles_bgtiles(&map)?;

    let fgtiles = fgtiles
        .map(Cow::Owned)
        .unwrap_or_else(|| Cow::Borrowed(vanilla_fgtiles_xml));
    let bgtiles = bgtiles
        .map(Cow::Owned)
        .unwrap_or_else(|| Cow::Borrowed(vanilla_bgtiles_xml));
    render_data.map_tileset = MapTileset::parse(&fgtiles, &bgtiles)?;

    let out = celesterender::render(render_data, asset_db, &map, RenderMapSettings::default())?;

    Ok(out)
}

fn main() -> Result<()> {
    render_modded_maps()?;
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
    // render_vanilla_maps(&_celeste)?;

    Ok(())
}

fn render_modded_maps() -> Result<()> {
    let celeste = CelesteInstallation::detect()?;

    let mut asset_db = AssetDb::new(ModLookup::in_folder(Path::new("downloads"), &celeste)?);

    let mut render_data = CelesteRenderData::base(&celeste)?;
    let vanilla_fgtiles_xml = celeste.read_to_string("Content/Graphics/ForegroundTiles.xml")?;
    let vanilla_bgtiles_xml = celeste.read_to_string("Content/Graphics/BackgroundTiles.xml")?;

    let downloaded_mods: Vec<(String, File)> = std::path::Path::new("downloads")
        .read_dir()
        .unwrap()
        .map(|x| {
            let path = x.unwrap().path();
            (
                path.file_name().unwrap().to_str().unwrap().to_owned(),
                File::open(path).unwrap(),
            )
        })
        .collect();

    let mut unknown: BTreeMap<String, u32> = BTreeMap::default();
    for (zip_name, m) in downloaded_mods {
        let mut zip = ModArchive::new(BufReader::new(m))?;

        if !zip_name.contains("Strawberry") {
            continue;
        }

        let mut maps = zip.list_map_files();
        maps.sort();

        let out_dir = PathBuf::from("out");
        std::fs::create_dir_all(&out_dir)?;

        for map_path in maps.iter() {
            let last_part = map_path.rsplit_once('/').unwrap().1;
            let img_path = out_dir
                .join(map_path.replace(['/'], "_"))
                .with_extension("png");

            if !map_path.contains("") {
                continue;
            }

            //if img_path.exists() {
            //continue;
            //}

            if map_path.contains("0-Calypta")
                || map_path.contains("Evilleaf")
                || map_path.contains("LeviathansRehearsal")
            {
                continue;
            }

            let res = render_map(
                &mut asset_db,
                &mut zip,
                &mut render_data,
                map_path,
                &vanilla_fgtiles_xml,
                &vanilla_bgtiles_xml,
            );
            match res {
                Err(e) => {
                    eprintln!("Error rendering {zip_name} {last_part}: {e}");
                }
                Ok(mut result) => {
                    result
                        .save_png(img_path, png::Compression::Default)
                        .context("saving png")?;

                    for (e, count) in result.unknown_entities {
                        *unknown.entry(e).or_default() += count;
                    }

                    eprintln!("Successfully rendered {last_part}");
                }
            }
        }

        // Ok(())
        // })?;
    }

    if !unknown.is_empty() {
        let mut unknown = unknown.iter().collect::<Vec<_>>();
        unknown.sort_by_key(|&(_, n)| std::cmp::Reverse(n));

        eprintln!(
            "{:2} unknown entities: ({} ...)",
            unknown.len(),
            unknown
                .iter()
                .take(50)
                .fold(String::new(), |mut acc, (name, num)| {
                    let _ = write!(&mut acc, "{num} {name} ");
                    acc
                })
        );
    }

    Ok(())
}

fn render_vanilla_maps(celeste: &CelesteInstallation) -> Result<()> {
    let out = PathBuf::from("out");
    std::fs::create_dir_all(&out)?;

    let start = Instant::now();

    let unknown_total = Arc::new(AtomicU32::new(0));

    let render_data = CelesteRenderData::vanilla(celeste)?;

    celeste
        .list_vanilla_maps()?
        .par_iter()
        .try_for_each::<_, Result<_>>(|map| {
            if !map.package.contains("") {
                return Ok(());
            }

            let start = Instant::now();
            let mut result = celesterender::render(
                &render_data,
                &mut AssetDb::empty(),
                map,
                RenderMapSettings {
                    layer: Layer::ALL,
                    include_room: &|room| room.name.starts_with(""),
                    status_update: &|_,_|{}
                },
            )?;
            let encode_start = Instant::now();
            result.save_png(out.join(&map.package).with_extension("png"), png::Compression::Default)?;
            let done = Instant::now();

            let render_duration = encode_start - start;
            let encode_duration = done - encode_start;

            if !result.unknown_entities.is_empty() {
                let mut unknown = result.unknown_entities.iter().collect::<Vec<_>>();
                unknown.sort_by_key(|&(_, n)| std::cmp::Reverse(n));

                let x = result.unknown_entities.values().copied().sum::<u32>();
                unknown_total.fetch_add(x, Ordering::Release);

                eprintln!(
                    "Took {:4.2}ms to render and {:4.2}ms to encode {:<20} Found {:2} unknown entities: ({} ...)",
                    render_duration.as_millis(),
                    encode_duration.as_millis(),
                    map.package,
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
        })?;

    println!(
        "Total {:>4.2?}ms, {} unknown",
        start.elapsed().as_millis(),
        unknown_total.load(Ordering::Acquire)
    );

    Ok(())
}
