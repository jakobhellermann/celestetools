use std::borrow::Cow;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::Instant;

use anyhow::{Context, Result};
use celesteloader::{archive::ModArchive, map::Map, CelesteInstallation};
use celesterender::{
    asset::{AssetDb, LookupAsset, ModLookup},
    CelesteRenderData, Layer,
};
use tiny_skia::Pixmap;

fn render_map<L: LookupAsset>(
    asset_db: &mut AssetDb<L>,
    zip: &mut ModArchive<BufReader<File>>,
    render_data: &mut CelesteRenderData,
    map_name: &str,
    vanilla_fgtiles_xml: &str,
    vanilla_bgtiles_xml: &str,
) -> Result<Pixmap> {
    let data = zip.read_file(map_name)?;
    let map = Map::parse(&data)?;

    let (fgtiles, bgtiles) = zip.map_fgtiles_bgtiles(&map)?;

    let fgtiles = fgtiles
        .map(Cow::Owned)
        .unwrap_or_else(|| Cow::Borrowed(vanilla_fgtiles_xml));
    let bgtiles = bgtiles
        .map(Cow::Owned)
        .unwrap_or_else(|| Cow::Borrowed(vanilla_bgtiles_xml));
    render_data.load_tilesets(&fgtiles, &bgtiles)?;

    let out = celesterender::render_with(render_data, asset_db, &map, Layer::ALL)?;

    Ok(out)
}

fn main() -> Result<()> {
    _render_modded_maps()?;

    let _celeste = CelesteInstallation::detect()?;
    // _render_vanilla_maps(&_celeste)?;

    Ok(())
}

fn _render_modded_maps() -> Result<()> {
    let celeste = CelesteInstallation::detect()?;

    let mods: Vec<File> = Path::new("downloads")
        .read_dir()
        .unwrap()
        .map(|x| File::open(x.unwrap().path()).unwrap())
        .collect();

    /*let mods =
    celesteloader::utils::list_dir_extension(&celeste.path.join("Mods"), "zip", |file| {
        File::open(file)
    })?;*/

    let mut mods = mods
        .into_iter()
        .map(|data| ModArchive::new(BufReader::new(data)))
        .collect::<Result<Vec<_>, _>>()?;
    let mut asset_db = AssetDb::new(ModLookup::new(mods.as_mut_slice(), &celeste));

    let mut render_data = CelesteRenderData::base(&celeste)?;

    let vanilla_fgtiles_xml = celeste.read_to_string("Content/Graphics/ForegroundTiles.xml")?;
    let vanilla_bgtiles_xml = celeste.read_to_string("Content/Graphics/BackgroundTiles.xml")?;

    let downloaded_mods: Vec<File> = Path::new("downloads")
        .read_dir()
        .unwrap()
        .map(|x| File::open(x.unwrap().path()).unwrap())
        .collect();
    for m in downloaded_mods {
        let mut zip = ModArchive::new(BufReader::new(m))?;

        let mut maps = zip.list_maps();
        maps.sort();

        let out_dir = PathBuf::from("out");
        std::fs::create_dir_all(&out_dir)?;

        for map_name in maps.iter() {
            let last_part = map_name.rsplit_once('/').unwrap().1;
            let img_path = out_dir.join(last_part).with_extension("png");

            if img_path.exists() {
                continue;
            }

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
                Ok(img) => {
                    img.save_png(img_path).context("saving png")?;
                    eprintln!("Successfully rendered {last_part}");
                }
            }
        }
    }

    Ok(())
}

fn _render_vanilla_maps(celeste: &CelesteInstallation) -> Result<()> {
    let out = PathBuf::from("out");
    std::fs::create_dir_all(&out)?;

    for map in celeste.vanilla_maps()?.iter()
    // .filter(|map| map.package.contains("Resort"))
    {
        let start = Instant::now();
        let pixmap = celesterender::render(celeste, &map, Layer::ALL)?;
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
