use anyhow::{anyhow, Context, Result};
use archive::ModArchive;
use atlas::AtlasMeta;
use std::{
    fs::File,
    io::BufReader,
    ops::ControlFlow,
    path::{Path, PathBuf},
};

pub mod archive;
pub mod atlas;
mod binaryreader;
pub mod cct_physics_inspector;
pub mod dialog;
pub mod map;
pub mod tileset;

mod steam_locate;

#[derive(Clone, Debug)]
pub struct CelesteInstallation {
    pub path: PathBuf,
}

impl CelesteInstallation {
    pub fn detect() -> Result<Self> {
        celeste_installation()
    }
    pub fn detect_multiple() -> Result<Vec<Self>> {
        Ok(celeste_installations()?)
    }

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

    pub fn vanilla_maps(&self) -> Result<Vec<map::Map>> {
        utils::list_dir_extension(&self.maps_dir(), "bin", |path| {
            let map = std::fs::read(path)
                .with_context(|| format!("failed to read map from '{}'", path.display()))?;
            let map = map::load_map(&map)?;
            Ok(map)
        })
    }

    pub fn list_atlases(&self) -> Result<Vec<AtlasMeta>> {
        let atlases =
            utils::list_dir_extension::<_, anyhow::Error>(&self.atlas_dir(), "meta", |path| {
                let meta = std::fs::read(path)?;
                let atlases = atlas::decode_atlas(&meta)?;
                Ok(atlases)
            })?
            .into_iter()
            .flatten()
            .collect();
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

    pub fn read_mod(&self, name: &str) -> Result<ModArchive> {
        let path = self.path.join("Mods").join(name).with_extension("zip");
        let archive = ModArchive::read(path)?;
        Ok(archive)
    }

    pub fn all_mods(&self) -> Result<Vec<ModArchive>> {
        let mods =
            utils::list_dir_extension(&self.path.join("Mods"), "zip", |file| File::open(file))?;
        let mods = mods
            .into_iter()
            .map(|data| ModArchive::new(BufReader::new(data)))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(mods)
    }

    pub fn list_mod_zips(&self) -> Result<Vec<PathBuf>> {
        utils::list_dir_extension(
            &self.path.join("Mods"),
            "zip",
            |path| Ok(path.to_path_buf()),
        )
    }

    pub fn mods_with<T>(
        &self,
        f: impl Fn(&str, ModArchive<BufReader<File>>) -> Result<T, anyhow::Error>,
    ) -> Result<Vec<T>> {
        utils::list_dir_extension(&self.path.join("Mods"), "zip", |path| {
            let filename = path.file_name().unwrap();
            let filename = filename
                .to_str()
                .ok_or_else(|| anyhow!("invalid utf8 in mod zip name"))?;

            let archive = ModArchive::new(BufReader::new(File::open(path)?))?;
            f(filename, archive)
        })
    }

    pub fn find_mod_with<T>(
        &self,
        f: impl Fn(&str, ModArchive<BufReader<File>>) -> Result<Option<T>, anyhow::Error>,
    ) -> Result<Option<T>> {
        utils::try_list_dir_extension(None, &self.path.join("Mods"), "zip", |_, path| {
            let filename = path.file_name().unwrap();
            let filename = filename
                .to_str()
                .ok_or_else(|| anyhow!("invalid utf8 in mod zip name"))?;

            let archive = ModArchive::new(BufReader::new(File::open(path)?))?;
            if let Some(res) = f(filename, archive)? {
                Ok(ControlFlow::Break(Some(res)))
            } else {
                Ok(ControlFlow::Continue(None))
            }
        })
    }

    pub fn physics_inspector(&self) -> PhysicsInspector {
        PhysicsInspector::new(self)
    }
}

fn celeste_installation() -> Result<CelesteInstallation> {
    let installations = celeste_installations()?;
    installations
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("no celeste installation found"))
}
fn celeste_installations() -> Result<Vec<CelesteInstallation>, std::io::Error> {
    let mut installations = Vec::new();

    if let Ok(steam) = steam_locate::locate_steam_dir() {
        let celeste = steam.join("steamapps/common/Celeste");

        if celeste.is_dir() && celeste.join("Celeste.dll").is_file() {
            installations.push(CelesteInstallation { path: celeste });
        }
    }

    Ok(installations)
}

pub mod utils {
    use std::{ops::ControlFlow, path::Path};

    pub fn list_dir_extension<T, E: From<std::io::Error>>(
        dir: &Path,
        extension: &str,
        mut f: impl FnMut(&Path) -> Result<T, E>,
    ) -> Result<Vec<T>, E> {
        try_list_dir_extension(Vec::new(), dir, extension, |mut acc, path| {
            let x = f(path)?;
            acc.push(x);
            Ok(ControlFlow::Continue(acc))
        })
    }

    pub fn try_list_dir_extension<A, E: From<std::io::Error>>(
        initial: A,
        dir: &Path,
        extension: &str,
        mut f: impl FnMut(A, &Path) -> Result<ControlFlow<A, A>, E>,
    ) -> Result<A, E> {
        let mut acc = initial;

        for entry in dir.read_dir()? {
            let entry = entry?;
            if !entry.file_type()?.is_file() {
                continue;
            }
            let path = entry.path();

            let is_extension = path.extension().map_or(false, |e| e == extension);
            if !is_extension {
                continue;
            }

            match f(acc, &path)? {
                ControlFlow::Continue(res) => acc = res,
                ControlFlow::Break(res) => {
                    acc = res;
                    break;
                }
            }
        }

        Ok(acc)
    }
}
