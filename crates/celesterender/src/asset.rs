use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use celesteloader::{archive::ModArchive, atlas::Sprite, CelesteInstallation};
use std::{fs::File, io::BufReader};
use tiny_skia::Pixmap;

use crate::CelesteRenderData;

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
impl<L: LookupAsset> AssetDb<L> {
    /*pub fn lookup_exact<'a>(
        &'a mut self,
        path: &str,
    ) -> Result<Option<(Vec<u8>, Option<&mut ModArchive>)>> {
        self.lookup_asset.lookup_exact(path)
    }*/

    pub fn lookup_gameplay<'a>(
        &'a mut self,
        cx: &'a CelesteRenderData,
        path: &str,
    ) -> Result<SpriteLocation<'a>> {
        if let Some(sprite) = cx.gameplay_sprites.get(path.trim_end_matches(".png")) {
            return Ok(SpriteLocation::Atlas(sprite));
        }

        if let Some(cached) = self.lookup_cache.get(path) {
            return Ok(SpriteLocation::Raw(cached));
        }

        if let Some(sprite) = self.lookup_asset.lookup_gameplay_png(path)? {
            let pixmap = Pixmap::decode_png(&sprite)
                .with_context(|| anyhow!("failed to decode {} as png", path))?;
            let a = self.lookup_cache.insert(path.to_owned(), Box::new(pixmap));
            return Ok(SpriteLocation::Raw(a));
        }

        Err(anyhow!("could not find '{}'", path))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SpriteLocation<'a> {
    Atlas(&'a Sprite),
    Raw(&'a Pixmap),
}
impl SpriteLocation<'_> {
    pub fn width(&self) -> i16 {
        match self {
            SpriteLocation::Atlas(sprite) => sprite.w,
            SpriteLocation::Raw(pixmap) => pixmap.width() as i16,
        }
    }
    pub fn height(&self) -> i16 {
        match self {
            SpriteLocation::Atlas(sprite) => sprite.h,
            SpriteLocation::Raw(pixmap) => pixmap.height() as i16,
        }
    }
    pub fn real_width(&self) -> i16 {
        match self {
            SpriteLocation::Atlas(sprite) => sprite.real_w,
            SpriteLocation::Raw(pixmap) => pixmap.width() as i16,
        }
    }
    pub fn real_height(&self) -> i16 {
        match self {
            SpriteLocation::Atlas(sprite) => sprite.real_h,
            SpriteLocation::Raw(pixmap) => pixmap.height() as i16,
        }
    }

    pub fn as_sprite(&self) -> Option<&Sprite> {
        match self {
            SpriteLocation::Atlas(sprite) => Some(sprite),
            SpriteLocation::Raw(_) => None,
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
