pub mod decode;

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

use decode::{Element, ValueType};

fn find_child_with_name<'a>(element: &'a Element, name: &'static str) -> Result<&'a Element<'a>> {
    element
        .find_child_with_name(name)
        .ok_or_else(|| Error::MissingElement(name))
}
impl<'a> Element<'a> {
    fn get_attr_or<T: ValueType<'a>>(&'a self, name: &'static str, default: T) -> Result<T> {
        let Some(value) = self.attributes.get(name) else { return Ok(default) };
        value.get::<T>().ok_or(Error::InvalidAttributeType {
            attribute: name,
            expected: std::any::type_name::<T>(),
            got: value.type_name(),
        })
    }
    fn get_attr<T: ValueType<'a>>(&'a self, name: &'static str) -> Result<T> {
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

    #[allow(dead_code)]
    fn get_attr_int_or(&'a self, name: &'static str, default: i32) -> Result<i32> {
        let Some(value) = self.attributes.get(name) else { return Ok(default) };
        value.get_int().ok_or(Error::InvalidAttributeType {
            attribute: name,
            expected: "integer",
            got: value.type_name(),
        })
    }
    fn get_attr_int(&'a self, name: &'static str) -> Result<i32> {
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

    fn get_attr_num_or(&'a self, name: &'static str, default: f32) -> Result<f32> {
        let Some(value) = self.attributes.get(name) else { return Ok(default) };
        value.get_number().ok_or(Error::InvalidAttributeType {
            attribute: name,
            expected: "number",
            got: value.type_name(),
        })
    }
    #[allow(dead_code)]
    fn get_attr_num(&'a self, name: &'static str) -> Result<f32> {
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
}

pub fn load_map(data: &[u8]) -> Result<Map> {
    let map = decode::decode_map(data).map_err(Error::Decode)?;

    let rooms = find_child_with_name(&map, "levels")?;
    let fillers = find_child_with_name(&map, "Filler")?;
    let _style = find_child_with_name(&map, "Style")?;
    let _fgstyle = map.find_child_with_name("Foregrounds");
    let _bgstyle = map.find_child_with_name("Backgrounds");

    let fillers = fillers
        .children
        .iter()
        .map(load_filler)
        .collect::<Result<Vec<_>>>()?;
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

fn load_filler<'a>(filler: &'a Element) -> Result<Filler> {
    Ok(Filler {
        position: (filler.get_attr_int("x")?, filler.get_attr_int("y")?),
        size: (filler.get_attr_int("w")?, filler.get_attr_int("h")?),
    })
}
fn load_room<'a>(room: &'a Element) -> Result<Room> {
    // decals, entities, triggers

    let fg_tiles_raw = find_child_with_name(room, "solids")?
        .get_attr_or::<&str>("innerText", "")?
        .to_owned();
    let bg_tiles_raw = find_child_with_name(room, "bg")?
        .get_attr_or::<&str>("innerText", "")?
        .to_owned();
    let obj_tiles_raw = room
        .find_child_with_name("obj")
        .map(|obj| obj.get_attr::<&str>("innerText"))
        .transpose()?
        .unwrap_or("")
        .to_owned();

    Ok(Room {
        name: room.get_attr::<&str>("name")?.to_string(),
        position: (room.get_attr_int("x")?, room.get_attr_int("y")?),
        size: (room.get_attr_int("x")?, room.get_attr_int("y")?),
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
    })
}
