use anyhow::{anyhow, Context, Result};
use archive::ModArchive;
use atlas::AtlasMeta;
use cct_physics_inspector::PhysicsInspector;
use map::Map;
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
pub mod save;
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

    pub fn data_dir(&self) -> PathBuf {
        if let Ok(var) = std::env::var("EVEREST_SAVEPATH") {
            if !var.is_empty() {
                return PathBuf::from(var);
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Some(xdg_data_home) = std::env::var_os("XDG_DATA_HOME") {
                return PathBuf::from(xdg_data_home).join("Celeste");
            }
            let home = std::env::var_os("HOME").unwrap();
            return PathBuf::from(home).join(".local/share/Celeste");
        }

        #[cfg(target_os = "macos")]
        {
            let home = std::env::var_os("HOME").unwrap();
            return PathBuf::from(home).join("Library/Application Support/Celeste");
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        return self.path.clone();
    }

    fn save_dir(&self) -> PathBuf {
        self.data_dir().join("Saves")
    }

    #[cfg(feature = "settings")]
    pub fn mod_settings(&self, mod_name: &str) -> Result<yaml_rust2::Yaml> {
        let path = self
            .save_dir()
            .join(format!("modsettings-{mod_name}.celeste"));
        let data = std::fs::read_to_string(&path)?;
        let mut parsed = yaml_rust2::YamlLoader::load_from_str(&data)?;
        if parsed.len() != 1 {
            return Err(anyhow::anyhow!(
                "'{}' modsettings contained {} yaml documents",
                mod_name,
                parsed.len()
            ));
        }

        Ok(parsed.remove(0))
    }

    pub fn saves(&self) -> Result<Vec<save::Save>> {
        let mut saves = Vec::new();

        let save_dir = self.save_dir();
        for item in save_dir.read_dir()? {
            let item = item?;
            let Some(i) = item
                .file_name()
                .to_str()
                .and_then(|file| file.strip_suffix(".celeste"))
                .and_then(|index| index.parse().ok())
            else {
                continue;
            };

            saves.push(save::Save::new(save_dir.clone(), i));
        }

        saves.sort();
        Ok(saves)
    }

    fn atlas_dir(&self) -> PathBuf {
        self.path.join("Content/Graphics/Atlases")
    }
    fn maps_dir(&self) -> PathBuf {
        self.path.join("Content/Maps")
    }

    pub fn physics_inspector(&self) -> PhysicsInspector {
        PhysicsInspector::new(self)
    }
}

impl CelesteInstallation {
    pub fn read_to_string(&self, path: impl AsRef<Path>) -> Result<String> {
        let str = std::fs::read_to_string(self.path.join(path))?;
        Ok(str)
    }
}

// maps
impl CelesteInstallation {
    pub fn vanilla_map(&self, map: &str) -> Result<map::Map> {
        let path = self.maps_dir().join(map).with_extension("bin");
        let map = std::fs::read(&path)
            .with_context(|| format!("failed to read map from '{}'", path.display()))?;
        let map = map::load_map(&map)?;
        Ok(map)
    }

    pub fn list_vanilla_maps(&self) -> Result<Vec<map::Map>> {
        utils::list_dir_extension(&self.maps_dir(), "bin", |path| {
            let map = std::fs::read(path)
                .with_context(|| format!("failed to read map from '{}'", path.display()))?;
            let map = map::load_map(&map)?;
            Ok(map)
        })
    }

    /// Vanilla sids prefixed by `Celeste/`
    pub fn find_map_by_map_bin(&self, map_bin: &str) -> Result<(Map, Option<ModArchive>)> {
        let result = if let Some(vanilla_sid) = map_bin.strip_prefix("Celeste/") {
            let map = self.vanilla_map(vanilla_sid)?;
            (map, None)
        } else {
            self.find_mod_with(|_, mut archive| {
                let map = archive.try_read_file(&format!("Maps/{map_bin}.bin"))?;
                let map = map.map(|data| Map::parse(&data)).transpose()?;

                Ok(map.map(|map| (map, Some(archive))))
            })?
            .with_context(|| anyhow!("could not find map .bin for {map_bin}"))?
        };
        Ok(result)
    }
}

// mods
impl CelesteInstallation {
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
}

// asset stuff
impl CelesteInstallation {
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
