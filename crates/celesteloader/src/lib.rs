use anyhow::{anyhow, Context, Result};
use atlas::AtlasMeta;
use std::path::{Path, PathBuf};

pub mod archive;
pub mod atlas;
mod binaryreader;
pub mod dialog;
pub mod map;
pub mod tileset;

mod steam_locate;

#[derive(Clone, Debug)]
pub struct CelesteInstallation {
    pub path: PathBuf,
}

impl CelesteInstallation {
    fn atlas_dir(&self) -> PathBuf {
        self.path.join("Content/Graphics/Atlases")
    }
    fn maps_dir(&self) -> PathBuf {
        self.path.join("Content/Maps")
    }

    pub fn read_to_string(&self, path: impl AsRef<Path>) -> Result<String> {
        let str = std::fs::read_to_string(self.path.join(path))?;
        Ok(str)
    }

    pub fn vanilla_map(&self, map: &str) -> Result<map::Map> {
        let path = self.maps_dir().join(map).with_extension("bin");
        let map = std::fs::read(&path)
            .with_context(|| format!("failed to read map from '{}'", path.display()))?;
        let map = map::load_map(&map)?;
        Ok(map)
    }

    pub fn list_atlases(&self) -> Result<Vec<AtlasMeta>> {
        let mut atlases = Vec::new();

        for entry in self.atlas_dir().read_dir()? {
            let entry = entry?;
            if !entry.file_type()?.is_file() {
                continue;
            }
            let path = entry.path();

            let is_meta = path
                .extension()
                .map_or(false, |extension| extension == "meta");
            if !is_meta {
                continue;
            }

            let meta = std::fs::read(&path)?;
            let a = atlas::decode_atlas(&meta)?;
            atlases.extend(a);
        }

        Ok(atlases)
    }

    pub fn gameplay_atlas(&self) -> Result<AtlasMeta> {
        let atlases = self.read_atlas_meta("Gameplay")?;
        atlases
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("Gameplay atlas not found"))
    }
    pub fn read_atlas_meta(&self, name: &str) -> Result<Vec<AtlasMeta>> {
        let atlas_dir = self.path.join("Content/Graphics/Atlases");

        let meta = std::fs::read(atlas_dir.join(name).with_extension("meta"))?;
        let atlases = atlas::decode_atlas(&meta)?;

        Ok(atlases)
    }
    pub fn decode_atlas_image(&self, meta: &AtlasMeta) -> Result<(u32, u32, Vec<u8>)> {
        let atlas_dir = self.path.join("Content/Graphics/Atlases");

        let data_path = atlas_dir.join(&meta.data).with_extension("data");

        let data = std::fs::read(data_path)?;
        let image = atlas::decode_data(&data)?;

        Ok(image)
    }
}

pub fn celeste_installation() -> Result<CelesteInstallation> {
    let installations = celeste_installations()?;
    installations
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("no celeste installation found"))
}
pub fn celeste_installations() -> Result<Vec<CelesteInstallation>, std::io::Error> {
    let mut installations = Vec::new();

    if let Ok(steam) = steam_locate::locate_steam_dir() {
        let celeste = steam.join("steamapps/common/Celeste");

        if celeste.is_dir() && celeste.join("Celeste.dll").is_file() {
            installations.push(CelesteInstallation { path: celeste });
        }
    }

    Ok(installations)
}
