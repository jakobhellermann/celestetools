use std::{fs::File, io::BufReader};

use anyhow::Result;
use celesteloader::{archive::ModArchive, CelesteInstallation};
use tiny_skia::Pixmap;

pub struct AssetDb<L> {
    pub(crate) lookup_asset: L,
    pub(crate) lookup_cache: elsa::FrozenMap<String, Box<Pixmap>>,
}
impl<L> AssetDb<L> {
    pub fn new(lookup: L) -> Self {
        AssetDb {
            lookup_asset: lookup,
            lookup_cache: Default::default(),
        }
    }
}

pub trait LookupAsset {
    fn lookup_exact(&mut self, path: &str) -> Result<Option<(Vec<u8>, Option<&mut ModArchive>)>>;

    fn lookup_gameplay_png(&mut self, path: &str) -> Result<Option<Vec<u8>>>;
}

impl<T: LookupAsset> LookupAsset for &mut T {
    fn lookup_exact(&mut self, path: &str) -> Result<Option<(Vec<u8>, Option<&mut ModArchive>)>> {
        (**self).lookup_exact(path)
    }

    fn lookup_gameplay_png(&mut self, path: &str) -> Result<Option<Vec<u8>>> {
        (**self).lookup_gameplay_png(path)
    }
}

pub struct NullLookup;
impl LookupAsset for NullLookup {
    fn lookup_exact(&mut self, _: &str) -> Result<Option<(Vec<u8>, Option<&mut ModArchive>)>> {
        Ok(None)
    }

    fn lookup_gameplay_png(&mut self, _: &str) -> Result<Option<Vec<u8>>> {
        Ok(None)
    }
}

pub struct ModLookup<R = BufReader<File>>(Vec<ModArchive<R>>, CelesteInstallation);

impl<R> ModLookup<R> {
    pub fn new(mods: Vec<ModArchive<R>>, celeste: &CelesteInstallation) -> Self {
        Self(mods, celeste.clone())
    }
}

impl ModLookup {
    pub fn all_mods(celeste: &CelesteInstallation) -> Result<Self> {
        let mods =
            celesteloader::utils::list_dir_extension(&celeste.path.join("Mods"), "zip", |file| {
                File::open(file)
            })?;
        let mods = mods
            .into_iter()
            .map(|data| ModArchive::new(BufReader::new(data)))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ModLookup::new(mods, celeste))
    }
}

impl LookupAsset for ModLookup {
    fn lookup_exact(&mut self, path: &str) -> Result<Option<(Vec<u8>, Option<&mut ModArchive>)>> {
        let vanilla_path = self.1.path.join("Content").join(path);
        if vanilla_path.exists() {
            let data = std::fs::read(vanilla_path)?;
            return Ok(Some((data, None)));
        }

        for archive in self.0.iter_mut() {
            if let Some(file) = archive.try_read_file(path)? {
                return Ok(Some((file, Some(archive))));
            }
        }

        Ok(None)
    }

    fn lookup_gameplay_png(&mut self, path: &str) -> Result<Option<Vec<u8>>> {
        let full = format!("Graphics/Atlases/Gameplay/{path}");
        let full_extension = format!("Graphics/Atlases/Gameplay/{path}.png");

        for archive in self.0.iter_mut() {
            if let Some(file) = archive.try_read_file(&full)? {
                return Ok(Some(file));
            }

            if let Some(file) = archive.try_read_file(&full_extension)? {
                return Ok(Some(file));
            }
        }

        for archive in self.0.iter_mut() {
            let file = archive
                .list_files()
                .find(|file| {
                    file.eq_ignore_ascii_case(&full) || file.eq_ignore_ascii_case(&full_extension)
                })
                .map(ToOwned::to_owned);

            if let Some(file) = file {
                let data = archive.read_file(&file)?;
                return Ok(Some(data));
            }
        }

        Ok(None)
    }
}
