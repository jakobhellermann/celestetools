use std::{
    borrow::Cow, cell::RefCell, fs::File, io::BufReader, path::PathBuf, rc::Rc, time::Instant,
};

use anyhow::{Context, Result};
use celesteloader::{archive::ModArchive, map::Map, CelesteInstallation};
use celesterender::{CelesteRenderData, Layer, LookupAsset};
use tiny_skia::Pixmap;

struct ModLookup<R>(Rc<RefCell<ModArchive<R>>>);

impl<R: std::io::Read + std::io::Seek> LookupAsset for ModLookup<R> {
    fn lookup(&self, path: &str) -> Result<Option<Vec<u8>>> {
        let mut archive = self.0.borrow_mut();

        let full = format!("Graphics/Atlases/Gameplay/{path}");
        match archive.read_file(&full) {
            Ok(data) => return Ok(Some(data)),
            Err(e) if e.is_file_not_found() => {}
            Err(e) => return Err(e).context(full),
        };

        let full = format!("Graphics/Atlases/Gameplay/{path}.png");
        match archive.read_file(&full) {
            Ok(data) => Ok(Some(data)),
            Err(e) if e.is_file_not_found() => Ok(None),
            Err(e) => Err(e).context(full),
        }
    }
}

fn render_map(
    celeste: &CelesteInstallation,
    lookup: &ModLookup<BufReader<File>>,
    zip: Rc<RefCell<ModArchive<BufReader<File>>>>,
    map_name: &str,
    vanilla_fgtiles_xml: &str,
    vanilla_bgtiles_xml: &str,
) -> Result<Pixmap> {
    let mut zip = zip.borrow_mut();
    let data = zip.read_file(&map_name)?;
    let map = Map::parse(&data)?;

    let fgtiles = map
        .meta
        .foreground_tiles
        .as_ref()
        .map(|p| {
            zip.read_file_string(p)
                .map(Cow::Owned)
                .with_context(|| format!("error reading {}", p))
        })
        .unwrap_or(Ok(Cow::Borrowed(vanilla_fgtiles_xml)))?;
    let bgtiles = map
        .meta
        .background_tiles
        .as_ref()
        .map(|p| zip.read_file_string(p).map(Cow::Owned))
        .unwrap_or(Ok(Cow::Borrowed(vanilla_bgtiles_xml)))
        .context("bgtiles")?;
    drop(zip);

    let mut render_data = CelesteRenderData::base(&celeste, &lookup)?;
    render_data.load_tilesets(&fgtiles, &bgtiles)?;

    let out = celesterender::render_with(&render_data, &map, Layer::ALL)?;

    Ok(out)
}

fn main() -> Result<()> {
    fastrand::seed(0);

    let celeste = celesteloader::celeste_installation()?;

    let vanilla_fgtiles_xml = celeste.read_to_string("Content/Graphics/ForegroundTiles.xml")?;
    let vanilla_bgtiles_xml = celeste.read_to_string("Content/Graphics/BackgroundTiles.xml")?;

    celeste.read_mod("StrawberryJam2021", |zip| {
        let mut maps = zip.list_maps();
        maps.sort();

        let zip = Rc::new(RefCell::new(zip));
        let lookup = ModLookup(zip.clone());

        let out_dir = PathBuf::from("out");
        std::fs::create_dir_all(&out_dir)?;

        for map_name in maps {
            let last_part = map_name.rsplit_once('/').unwrap().1;
            let img_path = out_dir.join(last_part).with_extension("png");

            if img_path.exists() {
                continue;
            }

            let res = render_map(
                &celeste,
                &lookup,
                zip.clone(),
                &map_name,
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
        Ok(())
    })?;

    //_render_vanilla_maps(&celeste)?;

    Ok(())
}

fn _render_vanilla_maps(celeste: &CelesteInstallation) -> Result<()> {
    let out = PathBuf::from("out");
    std::fs::create_dir_all(&out)?;

    // for map in celeste.vanilla_maps()? {

    let map = celesteloader::map::Map::open("/home/jakob/Downloads/0-Intro.bin")?;

    for map in [map] {
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
