pub mod decode;
pub mod utils;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    Decode(decode::Error),
    MissingElement(&'static str),
    MissingAttribute {
        attribute: &'static str,
        element_name: String,
    },
    InvalidAttributeType {
        attribute: &'static str,
        expected: &'static str,
        got: &'static str,
    },
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Decode(error) => Some(error),
            _ => None,
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Decode(e) => write!(f, "failed to decode map: {}", e),
            Error::MissingElement(e) => write!(f, "could not find element `{e}`"),
            Error::MissingAttribute {
                attribute,
                element_name,
            } => write!(
                f,
                "could not find attribute `{attribute}` on element `{element_name}`"
            ),
            Error::InvalidAttributeType {
                attribute,
                expected,
                got,
            } => write!(
                f,
                "expected attribute `{attribute}` to have type `{expected}`, got `{got}`"
            ),
        }
    }
}

use std::path::Path;

use decode::{Element, ValueType};

impl<'a> Element<'a> {
    pub fn child_with_name(&'a self, name: &'static str) -> Result<&'a Element<'a>> {
        self.find_child_with_name(name)
            .ok_or(Error::MissingElement(name))
    }

    pub fn try_get_attr<T: ValueType<'a>>(&'a self, name: &'static str) -> Result<Option<T>> {
        let Some(value) = self.attributes.get(name) else {
            return Ok(None);
        };
        value
            .get::<T>()
            .ok_or(Error::InvalidAttributeType {
                attribute: name,
                expected: std::any::type_name::<T>(),
                got: value.type_name(),
            })
            .map(Some)
    }

    pub fn get_attr_or<T: ValueType<'a>>(&'a self, name: &'static str, default: T) -> Result<T> {
        let Some(value) = self.attributes.get(name) else {
            return Ok(default);
        };
        value.get::<T>().ok_or(Error::InvalidAttributeType {
            attribute: name,
            expected: std::any::type_name::<T>(),
            got: value.type_name(),
        })
    }
    pub fn get_attr<T: ValueType<'a>>(&'a self, name: &'static str) -> Result<T> {
        let value = self
            .attributes
            .get(name)
            .ok_or_else(|| Error::MissingAttribute {
                attribute: name,
                element_name: self.name.to_owned(),
            })?;
        value.get::<T>().ok_or(Error::InvalidAttributeType {
            attribute: name,
            expected: std::any::type_name::<T>(),
            got: value.type_name(),
        })
    }

    pub fn get_attr_int_or(&'a self, name: &'static str, default: i32) -> Result<i32> {
        let Some(value) = self.attributes.get(name) else {
            return Ok(default);
        };
        value.get_int().ok_or(Error::InvalidAttributeType {
            attribute: name,
            expected: "integer",
            got: value.type_name(),
        })
    }
    pub fn get_attr_int(&'a self, name: &'static str) -> Result<i32> {
        let value = self
            .attributes
            .get(name)
            .ok_or_else(|| Error::MissingAttribute {
                attribute: name,
                element_name: self.name.to_owned(),
            })?;
        value.get_int().ok_or(Error::InvalidAttributeType {
            attribute: name,
            expected: "integer",
            got: value.type_name(),
        })
    }
    pub fn try_get_attr_int(&'a self, name: &'static str) -> Result<Option<i32>> {
        let Some(value) = self.attributes.get(name) else {
            return Ok(None);
        };
        value
            .get_int()
            .ok_or(Error::InvalidAttributeType {
                attribute: name,
                expected: "integer",
                got: value.type_name(),
            })
            .map(Some)
    }

    pub fn get_attr_num_or(&'a self, name: &'static str, default: f32) -> Result<f32> {
        let Some(value) = self.attributes.get(name) else {
            return Ok(default);
        };
        value.get_number().ok_or(Error::InvalidAttributeType {
            attribute: name,
            expected: "number",
            got: value.type_name(),
        })
    }
    pub fn get_attr_num(&'a self, name: &'static str) -> Result<f32> {
        let value = self
            .attributes
            .get(name)
            .ok_or_else(|| Error::MissingAttribute {
                attribute: name,
                element_name: self.name.to_owned(),
            })?;
        value.get_number().ok_or(Error::InvalidAttributeType {
            attribute: name,
            expected: "integer",
            got: value.type_name(),
        })
    }
}

#[derive(Debug)]
pub struct Map {
    pub package: String,
    pub rooms: Vec<Room>,
    pub fillers: Vec<Filler>,
    // TODO style
    pub meta: Metadata,
}

#[derive(Debug)]
pub struct Metadata {
    // ...
    pub icon: Option<String>,
    pub override_a_site_meta: bool,
    pub intro_type: String,
    pub background_tiles: Option<String>,
    pub foreground_tiles: Option<String>,
}

#[derive(Debug)]
pub struct Filler {
    pub position: (i32, i32),
    pub size: (i32, i32),
}

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    /// Top left
    pub position: Pos,
    pub size: (u32, u32),
}

impl std::fmt::Display for Bounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{} {},{}",
            self.position.x, self.position.y, self.size.0, self.size.1
        )
    }
}

#[derive(Debug)]
pub struct Room {
    pub name: String,
    pub bounds: Bounds,
    pub fg_tiles_raw: String,
    pub bg_tiles_raw: String,
    pub obj_tiles_raw: String,
    pub scenery_fg_raw: String,
    pub scenery_bg_raw: String,

    // TODO music decals entities triggers
    pub dark: bool,
    pub space: bool,
    pub underwater: bool,
    pub whisper: bool,
    pub disable_down_transition: bool,

    pub wind_pattern: String,
    pub color: u8,

    pub camera_offset: (f32, f32),

    pub entities: Vec<Entity>,
    pub triggers: Vec<Trigger>,

    pub decals_bg: Vec<Decal>,
    pub decals_fg: Vec<Decal>,
}

#[derive(Debug)]
pub struct Entity {
    pub id: i32,
    pub position: (f32, f32),
    pub name: String,
}

#[derive(Debug)]
pub struct Trigger {
    pub id: Option<i32>,
    pub position: (f32, f32),
    pub extents: (i32, i32),
    pub name: String,
}

#[derive(Debug)]
pub struct Decal {
    pub x: f32,
    pub y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32,
    pub texture: String,
}

pub fn load_map(data: &[u8]) -> Result<Map> {
    let map = decode::decode_map(data).map_err(Error::Decode)?;
    load_map_from_element(&map)
}

pub fn load_map_from_element(map: &Element<'_>) -> Result<Map> {
    let rooms = map.child_with_name("levels")?;
    let fillers = map.find_child_with_name("Filler");
    let _style = map.child_with_name("Style")?;
    let _fgstyle = map.find_child_with_name("Foregrounds");
    let _bgstyle = map.find_child_with_name("Backgrounds");

    let fillers = fillers
        .map(|fillers| {
            fillers
                .children
                .iter()
                .map(load_filler)
                .collect::<Result<Vec<_>>>()
        })
        .unwrap_or(Ok(Vec::new()))?;
    let rooms = rooms
        .children
        .iter()
        .map(load_room)
        .collect::<Result<Vec<_>>>()?;

    let meta = map
        .find_child_with_name("meta")
        .map(|meta| load_metadata(meta))
        .unwrap_or_else(|| {
            Ok(Metadata {
                icon: None,
                override_a_site_meta: false,
                intro_type: "todo".into(),
                background_tiles: None,
                foreground_tiles: None,
            })
        })?;

    Ok(Map {
        package: map.get_attr::<&str>("package")?.to_string(),
        rooms,
        fillers,
        meta,
    })
}

fn load_metadata(metadata: &Element) -> Result<Metadata> {
    Ok(Metadata {
        icon: metadata
            .try_get_attr::<&str>("Icon")?
            .map(ToOwned::to_owned),
        override_a_site_meta: metadata.get_attr_or::<bool>("OverrideASideMeta", false)?,
        intro_type: metadata.get_attr::<&str>("IntroType")?.to_owned(),
        foreground_tiles: metadata
            .try_get_attr::<&str>("ForegroundTiles")?
            .map(|str| str.replace('\\', "/")),
        background_tiles: metadata
            .try_get_attr::<&str>("BackgroundTiles")?
            .map(|str| str.replace('\\', "/")),
    })
}
fn load_filler(filler: &Element) -> Result<Filler> {
    Ok(Filler {
        position: (filler.get_attr_int("x")?, filler.get_attr_int("y")?),
        size: (filler.get_attr_int("w")?, filler.get_attr_int("h")?),
    })
}
fn load_room(room: &Element) -> Result<Room> {
    let fg_tiles_raw = room
        .child_with_name("solids")?
        .get_attr_or::<&str>("innerText", "")?
        .to_owned();
    let bg_tiles_raw = room
        .child_with_name("bg")?
        .get_attr_or::<&str>("innerText", "")?
        .to_owned();
    let obj_tiles_raw = room
        .find_child_with_name("obj")
        .map(|obj| obj.get_attr::<&str>("innerText"))
        .transpose()?
        .unwrap_or("")
        .to_owned();
    let scenery_fg_raw = room
        .find_child_with_name("fgtiles")
        .map(|obj| obj.get_attr_or::<&str>("innerText", ""))
        .transpose()?
        .unwrap_or("")
        .to_owned();
    let scenery_bg_raw = room
        .find_child_with_name("bgtiles")
        .map(|obj| obj.get_attr_or::<&str>("innerText", ""))
        .transpose()?
        .unwrap_or("")
        .to_owned();
    let entities = room
        .find_child_with_name("entities")
        .map(|entities| {
            entities
                .children
                .iter()
                .map(|entity| {
                    let id = entity.get_attr_int("id")?;
                    let x = entity.get_attr_num("x")?;
                    let y = entity.get_attr_num("y")?;

                    Ok(Entity {
                        id,
                        position: (x, y),
                        name: entity.name.to_owned(),
                    })
                })
                .collect::<Result<Vec<_>>>()
        })
        .unwrap_or(Ok(Vec::new()))?;

    let triggers = room
        .find_child_with_name("triggers")
        .map(|triggers| {
            triggers
                .children
                .iter()
                .map(|trigger| {
                    let id = trigger.try_get_attr_int("id")?;
                    let x = trigger.get_attr_num("x")?;
                    let y = trigger.get_attr_num("y")?;
                    let width = trigger.get_attr_int("width")?;
                    let height = trigger.get_attr_int("height")?;

                    Ok(Trigger {
                        id,
                        position: (x, y),
                        extents: (width, height),
                        name: trigger.name.to_owned(),
                    })
                })
                .collect::<Result<Vec<_>>>()
        })
        .unwrap_or(Ok(Vec::new()))?;

    let decals_bg = room
        .find_child_with_name("bgdecals")
        .map(|bgdecals| {
            bgdecals
                .children
                .iter()
                .map(load_decal)
                .collect::<Result<Vec<_>>>()
        })
        .unwrap_or(Ok(Vec::new()))?;

    let decals_fg = room
        .find_child_with_name("fgdecals")
        .map(|fgdecals| {
            fgdecals
                .children
                .iter()
                .map(load_decal)
                .collect::<Result<Vec<_>>>()
        })
        .unwrap_or(Ok(Vec::new()))?;

    let position = Pos {
        x: room.get_attr_int("x")?,
        y: room.get_attr_int("y")?,
    };
    let size = (
        room.get_attr_int("width")?.try_into().unwrap(),
        room.get_attr_int("height")?.try_into().unwrap(),
    );

    Ok(Room {
        name: room.get_attr::<&str>("name")?.to_string(),
        bounds: Bounds { position, size },
        fg_tiles_raw,
        bg_tiles_raw,
        obj_tiles_raw,
        scenery_fg_raw,
        scenery_bg_raw,
        dark: room.get_attr_or("dark", false)?,
        space: room.get_attr_or("space", false)?,
        underwater: room.get_attr_or("underwater", false)?,
        whisper: room.get_attr_or("whisper", false)?,
        disable_down_transition: room.get_attr_or("disableDownTransition", false)?,
        wind_pattern: room.get_attr_or("windPattern", "")?.to_string(),
        color: room.get_attr_or("color", 0)?,
        camera_offset: (
            room.get_attr_num_or("cameraOffsetX", 0.0)?,
            room.get_attr_num_or("cameraOffsetY", 0.0)?,
        ),
        entities,
        triggers,
        decals_bg,
        decals_fg,
    })
}

fn load_decal(decal: &Element) -> Result<Decal> {
    if decal.attributes.contains_key("jx") {
        todo!()
    }
    if decal.attributes.contains_key("justificationX") {
        todo!()
    }
    if decal.attributes.contains_key("depth") {
        todo!()
    }

    Ok(Decal {
        x: decal.get_attr_num("x")?,
        y: decal.get_attr_num("y")?,
        scale_x: decal.get_attr_num("scaleX")?,
        scale_y: decal.get_attr_num("scaleY")?,
        rotation: decal.get_attr_num_or("rotation", 0.0)?,
        texture: decal.get_attr::<&str>("texture")?.replace('\\', "/"),
    })
}

impl Map {
    pub fn parse(data: &[u8]) -> Result<Self> {
        load_map(data)
    }
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let data = std::fs::read(path).unwrap();
        load_map(&data)
    }

    pub fn bounds(&self) -> Bounds {
        self.rooms
            .iter()
            .map(|r| r.bounds)
            .reduce(Bounds::join)
            .expect("map has no rooms")
    }
}

impl Room {
    pub fn entities_by_name<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &Entity> {
        self.entities
            .iter()
            .filter(move |entity| entity.name == name)
    }
    pub fn find_entity_by_name(&self, name: &str) -> Option<&Entity> {
        self.entities
            .iter()
            .find(move |entity| (entity.name == name))
    }
}
impl Bounds {
    pub fn r(self) -> i32 {
        self.position.x + self.size.0 as i32
    }
    pub fn b(self) -> i32 {
        self.position.y + self.size.1 as i32
    }

    pub fn join(self, other: Bounds) -> Self {
        let x = self.position.x.min(other.position.x);
        let y = self.position.y.min(other.position.y);

        let r = self.r().max(other.r());
        let b = self.b().max(other.b());

        Bounds {
            position: Pos { x, y },
            size: ((r - x) as u32, (b - y) as u32),
        }
    }

    pub fn size_tiles(&self) -> (u32, u32) {
        (self.size.0 / 8, self.size.1 / 8)
    }
    pub fn position_tiles(&self) -> (i32, i32) {
        (self.position.x / 8, self.position.y / 8)
    }
}

impl Pos {
    pub fn offset(self, x: i32, y: i32) -> Self {
        Pos {
            x: self.x + x,
            y: self.y + y,
        }
    }
    pub fn offset_tile(self, x: i32, y: i32) -> Self {
        self.offset(x * 8, y * 8)
    }

    pub fn tile_rect(self) -> Bounds {
        Bounds {
            position: self,
            size: (8, 8),
        }
    }
}
