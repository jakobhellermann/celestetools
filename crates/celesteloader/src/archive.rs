use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use zip::{result::ZipError, ZipArchive};

use crate::{dialog::Dialog, map::Map};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    Zip(ZipError),
    IO(std::io::Error),
    Map(crate::map::Error),
}
impl Error {
    pub fn is_file_not_found(&self) -> bool {
        matches!(self, Error::Zip(ZipError::FileNotFound))
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Zip(error) => Some(error),
            Error::IO(error) => Some(error),
            Error::Map(error) => Some(error),
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Zip(e) => write!(f, "error reading zip archive: {e}"),
            Error::IO(e) => write!(f, "IO error: {e}"),
            Error::Map(e) => write!(f, "failed to decode map: {e}"),
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
impl From<crate::map::Error> for Error {
    fn from(error: crate::map::Error) -> Self {
        Error::Map(error)
    }
}

pub struct ModArchive<R = BufReader<File>> {
    archive: ZipArchive<R>,
}
impl ModArchive<BufReader<File>> {
    pub fn read(path: impl AsRef<Path>) -> Result<ModArchive> {
        let file = BufReader::new(File::open(path).map_err(Error::IO)?);
        let archive = ModArchive::new(file)?;
        Ok(archive)
    }
}

impl<R: std::io::Read + std::io::Seek> ModArchive<R> {
    pub fn new(reader: R) -> Result<Self, Error> {
        let zip = ZipArchive::new(reader)?;
        Ok(ModArchive { archive: zip })
    }

    pub fn list_files(&self) -> impl Iterator<Item = &str> {
        self.archive.file_names()
    }

    pub fn read_file(&mut self, name: &str) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.archive.by_name(name)?.read_to_end(&mut buf)?;
        Ok(buf)
    }

    pub fn try_read_file(&mut self, name: &str) -> Result<Option<Vec<u8>>> {
        let mut buf = Vec::new();

        match self.archive.by_name(name) {
            Ok(mut read) => {
                read.read_to_end(&mut buf)?;
                Ok(Some(buf))
            }
            Err(ZipError::FileNotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn read_file_string(&mut self, name: &str) -> Result<String> {
        let mut buf = String::new();
        self.archive.by_name(name)?.read_to_string(&mut buf)?;
        Ok(buf)
    }
}

impl<R: std::io::Read + std::io::Seek> ModArchive<R> {
    pub fn list_map_files(&self) -> Vec<String> {
        self.list_files()
            .filter(|file| file.starts_with("Maps") && file.ends_with(".bin"))
            .map(|s| s.to_owned())
            .collect()
    }

    pub fn read_map(&mut self, map_path: &str) -> Result<Map> {
        let data = self.read_file(map_path)?;
        let map = Map::parse(&data)?;
        Ok(map)
    }

    pub fn get_dialog(&mut self, lang: &str) -> Result<Dialog> {
        self.try_get_dialog(lang)
            .and_then(|val| val.ok_or(Error::Zip(ZipError::FileNotFound)))
    }

    pub fn try_get_dialog(&mut self, lang: &str) -> Result<Option<Dialog>> {
        let result = self.archive.by_name(&format!("Dialog/{lang}.txt"));
        let file = match result {
            Ok(file) => Some(file),
            Err(_) => {
                drop(result);
                match self
                    .archive
                    .by_name(&format!("Dialog/{}.txt", lang.to_ascii_lowercase()))
                {
                    Ok(file) => Some(file),
                    Err(ZipError::FileNotFound) => None,
                    Err(e) => return Err(e.into()),
                }
            }
        };
        file.map(Dialog::from_read).transpose().map_err(Error::IO)
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

    pub fn map_fgtiles_bgtiles(&mut self, map: &Map) -> Result<(Option<String>, Option<String>)> {
        let fgtiles = map
            .meta
            .foreground_tiles
            .as_ref()
            .map(|path| self.read_file_string(path))
            .transpose()?;
        let bgtiles = map
            .meta
            .background_tiles
            .as_ref()
            .map(|path| self.read_file_string(path))
            .transpose()?;
        Ok((fgtiles, bgtiles))
    }
}
