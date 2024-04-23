use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub struct Save {
    i: u32,
    save_dir: PathBuf,
}

impl Ord for Save {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.i.cmp(&other.i)
    }
}

impl PartialOrd for Save {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.i.cmp(&other.i))
    }
}

impl Save {
    pub(crate) fn new(save_dir: PathBuf, i: u32) -> Self {
        Save { save_dir, i }
    }

    pub fn index(&self) -> u32 {
        self.i
    }

    pub fn xml<T>(&self, f: impl FnOnce(roxmltree::Document<'_>) -> T) -> Result<T> {
        let data = std::fs::read_to_string(self.save_dir.join(format!("{}.celeste", self.i)))?;
        let document = roxmltree::Document::parse(&data)?;
        let ret = f(document);
        Ok(ret)
    }
}
