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

use decode::{Element, ValueType};

impl<'a> Element<'a> {
    pub fn child_with_name(&'a self, name: &'static str) -> Result<&'a Element<'a>> {
        self.find_child_with_name(name)
            .ok_or(Error::MissingElement(name))
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
}

#[derive(Debug)]
pub struct Filler {
    pub position: (i32, i32),
    pub size: (i32, i32),
}

#[derive(Debug)]
pub struct Room {
    pub name: String,
    pub position: (i32, i32),
    pub size: (i32, i32),
    pub fg_tiles_raw: String,
    pub bg_tiles_raw: String,
    pub obj_tiles_raw: String,

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
}

#[derive(Debug)]
pub struct Entity {
    pub id: i32,
    pub position: (f32, f32),
    pub name: String,
}

#[derive(Debug)]
pub struct Trigger {
    pub id: i32,
    pub position: (f32, f32),
    pub extents: (i32, i32),
    pub name: String,
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

    Ok(Map {
        package: map.get_attr::<&str>("package")?.to_string(),
        rooms,
        fillers,
    })
}

fn load_filler(filler: &Element) -> Result<Filler> {
    Ok(Filler {
        position: (filler.get_attr_int("x")?, filler.get_attr_int("y")?),
        size: (filler.get_attr_int("w")?, filler.get_attr_int("h")?),
    })
}
fn load_room(room: &Element) -> Result<Room> {
    // decals, entities, triggers

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
                    let id = trigger.get_attr_int("id")?;
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

    Ok(Room {
        name: room.get_attr::<&str>("name")?.to_string(),
        position: (room.get_attr_int("x")?, room.get_attr_int("y")?),
        size: (room.get_attr_int("width")?, room.get_attr_int("height")?),
        fg_tiles_raw,
        bg_tiles_raw,
        obj_tiles_raw,
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
    })
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
