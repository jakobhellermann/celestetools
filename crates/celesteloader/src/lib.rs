use std::path::PathBuf;

pub mod archive;
pub mod dialog;
pub mod map;

mod steam_locate;


#[derive(Clone)]
pub struct CelesteInstallation {
    pub path: PathBuf,
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
