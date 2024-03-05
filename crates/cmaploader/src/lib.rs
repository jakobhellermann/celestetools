use std::path::PathBuf;

pub mod archive;
pub mod dialog;
pub mod map;

#[derive(Clone)]
pub struct CelesteInstallation {
    pub path: PathBuf,
}

pub fn celeste_installations() -> Result<Vec<CelesteInstallation>, std::io::Error> {
    Ok(vec![CelesteInstallation {
        path: PathBuf::from("C:/Program Files (x86)/Steam/steamapps/common/Celeste"),
    }])
}
