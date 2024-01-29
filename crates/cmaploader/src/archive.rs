use std::io::Read;

use zip::{result::ZipError, ZipArchive};

use crate::dialog::Dialog;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    Zip(ZipError),
    IO(std::io::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Zip(error) => Some(error),
            Error::IO(error) => Some(error),
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Zip(e) => write!(f, "error reading zip archive: {e}"),
            Error::IO(e) => write!(f, "IO error: {e}"),
        }
    }
}

impl From<ZipError> for Error {
    fn from(error: ZipError) -> Self {
        Error::Zip(error)
    }
}
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}

pub struct ModArchive<R> {
    archive: ZipArchive<R>,
}

impl<R: std::io::Read + std::io::Seek> ModArchive<R> {
    pub fn new(reader: R) -> Result<Self, Error> {
        let zip = ZipArchive::new(reader)?;
        Ok(ModArchive { archive: zip })
    }

    pub fn get_dialog(&mut self, lang: &str) -> Result<Dialog> {
        let result = self.archive.by_name(&format!("Dialog/{lang}.txt"));
        let file = match result {
            Ok(file) => file,
            Err(_) => {
                drop(result);
                match self
                    .archive
                    .by_name(&format!("Dialog/{}.txt", lang.to_ascii_lowercase()))
                {
                    Ok(file) => file,
                    Err(e) => return Err(e.into()),
                }
            }
        };

        Dialog::from_read(file).map_err(Error::IO)
    }

    pub fn everest_yaml(&mut self) -> Result<String> {
        // this is terrible code but I don't know how to get around the borrow checker match limitations
        let result = self.archive.by_name("everest.yaml");
        let mut file = match result {
            Ok(file) => file,
            Err(_) => {
                drop(result);
                let result = self.archive.by_name("everest.yml");
                match result {
                    Ok(file) => file,
                    Err(_) => {
                        drop(result);
                        let result = self.archive.by_name("Everest.yaml");
                        match result {
                            Ok(file) => file,
                            Err(_) => {
                                drop(result);
                                let result = self.archive.by_name("Everest.yml");
                                match result {
                                    Ok(file) => file,
                                    Err(e) => {
                                        return Err(e.into());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };

        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Ok(buf)
    }

    pub fn is_collab(&mut self) -> bool {
        self.archive.by_name("CollabUtils2CollabID.txt").is_ok()
    }

    pub fn read_file(&mut self, name: &str) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.archive.by_name(name)?.read_to_end(&mut buf)?;
        Ok(buf)
    }

    pub fn list_files(&self) -> impl Iterator<Item = &str> {
        self.archive.file_names()
    }
}
