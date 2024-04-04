use anyhow::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Data;

#[derive(Debug, Deserialize, Clone)]
pub struct Tileset {
    pub id: char,
    pub copy: Option<char>,
    pub path: String,
    pub ignores: Option<String>,

    #[serde(default)]
    pub set: Vec<Set>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Set {
    pub mask: String,
    pub tiles: String,
}

pub fn parse_tilesets(str: &str) -> Result<Vec<Tileset>, Error> {
    let document = roxmltree::Document::parse(str)?;
    let data = document.root_element();
    let tilesets = data.children();

    let tilesets = tilesets
        .filter(|node| node.is_element())
        .map(|tileset| serde_roxmltree::from_node(tileset))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(tilesets)
}
