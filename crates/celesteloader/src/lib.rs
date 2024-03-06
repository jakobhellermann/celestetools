use anyhow::Result;
use atlas::AtlasMeta;
use std::path::PathBuf;

pub mod archive;
pub mod atlas;
mod binaryreader;
pub mod dialog;
pub mod map;

mod steam_locate;

#[derive(Clone, Debug)]
pub struct CelesteInstallation {
    pub path: PathBuf,
}

impl CelesteInstallation {
    fn atlas_dir(&self) -> PathBuf {
        self.path.join("Content/Graphics/Atlases")
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

    pub fn read_atlas_meta(&self, name: &str) -> Result<Vec<AtlasMeta>> {
        let atlas_dir = self.path.join("Content/Graphics/Atlases");

        let meta = std::fs::read(atlas_dir.join(name).with_extension("meta"))?;
        let atlases = atlas::decode_atlas(&meta)?;

        Ok(atlases)
    }
    pub fn decode_atlas_image(&self, meta: &AtlasMeta) -> Result<image::RgbaImage> {
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
        .ok_or_else(|| anyhow::anyhow!("no celeste installation found"))
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
