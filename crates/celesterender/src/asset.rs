use anyhow::Result;
use celesteloader::archive::ModArchive;
use tiny_skia::Pixmap;

pub struct AssetDb<L> {
    pub lookup_asset: L,
    pub lookup_cache: elsa::FrozenMap<String, Box<Pixmap>>,
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
    fn lookup(&mut self, path: &str) -> Result<Option<Vec<u8>>>;
}

impl<T: LookupAsset> LookupAsset for &mut T {
    fn lookup(&mut self, path: &str) -> Result<Option<Vec<u8>>> {
        (**self).lookup(path)
    }
}

pub struct NullLookup;
impl LookupAsset for NullLookup {
    fn lookup(&mut self, _: &str) -> Result<Option<Vec<u8>>> {
        Ok(None)
    }
}

pub struct ModLookup<'a, R>(&'a mut [ModArchive<R>], u32, u32, u32);

impl<'a, R> ModLookup<'a, R> {
    pub fn new(mods: &'a mut [ModArchive<R>]) -> Self {
        Self(mods, 0, 0, 0)
    }
}

impl<'a, R: std::io::Read + std::io::Seek> LookupAsset for ModLookup<'a, R> {
    fn lookup(&mut self, path: &str) -> Result<Option<Vec<u8>>> {
        let full = format!("Graphics/Atlases/Gameplay/{path}");
        let full_extension = format!("Graphics/Atlases/Gameplay/{path}.png");

        for archive in self.0.iter_mut() {
            if let Some(file) = archive.try_read_file(&full)? {
                self.1 += 1;
                return Ok(Some(file));
            }

            if let Some(file) = archive.try_read_file(&full_extension)? {
                self.2 += 1;
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
                self.3 += 1;
                return Ok(Some(data));
            }
        }

        Ok(None)
    }
}
