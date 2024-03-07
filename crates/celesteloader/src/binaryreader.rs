use std::{borrow::Cow, collections::HashMap};

#[derive(Debug)]
pub struct Element<'a> {
    pub name: &'a str,
    pub attributes: HashMap<&'a str, Value<'a>>,
    pub children: Vec<Element<'a>>,
}
impl<'a> Element<'a> {
    pub fn find_child_with_name(&self, name: &str) -> Option<&Element> {
        self.children.iter().find(|child| child.name == name)
    }
}

#[derive(Debug)]
pub enum Value<'a> {
    Bool(bool),
    U8(u8),
    I16(i16),
    I32(i32),
    F32(f32),
    String(Cow<'a, str>),
}
impl<'a> Value<'a> {
    pub fn get_int(&'a self) -> Option<i32> {
        Some(match *self {
            Value::U8(val) => val as i32,
            Value::I16(val) => val as i32,
            Value::I32(val) => val,
            _ => return None,
        })
    }
    pub fn get_number(&'a self) -> Option<f32> {
        Some(match *self {
            Value::U8(val) => val as f32,
            Value::I16(val) => val as f32,
            Value::I32(val) => val as f32,
            Value::F32(val) => val,
            _ => return None,
        })
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Bool(_) => "bool",
            Value::U8(_) => "u8",
            Value::I16(_) => "i16",
            Value::I32(_) => "i32",
            Value::F32(_) => "f32",
            Value::String(_) => "str",
        }
    }
    pub fn get<T: ValueType<'a>>(&'a self) -> Option<T> {
        T::get(self)
    }
    pub fn get_or<T: ValueType<'a>>(&'a self, default: T) -> T {
        T::get(self).unwrap_or(default)
    }
}

pub trait ValueType<'a>
where
    Self: Sized,
{
    fn get(value: &'a Value<'a>) -> Option<Self>;
}
macro_rules! impl_valuetype {
    ($ty:ty: $kind:ident) => {
        impl ValueType<'_> for $ty {
            fn get(value: &Value<'_>) -> Option<Self> {
                match *value {
                    Value::$kind(val) => Some(val),
                    _ => None,
                }
            }
        }
    };
}
impl_valuetype!(bool: Bool);
impl_valuetype!(u8: U8);
impl_valuetype!(i16: I16);
impl_valuetype!(i32: I32);
impl_valuetype!(f32: F32);
impl<'a> ValueType<'a> for &'a str {
    fn get(value: &'a Value<'a>) -> Option<Self> {
        match value {
            Value::String(val) => Some(val),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    EOF,
    InvalidHeader,
    InvalidUTF8,
    InvalidLookup,
    InvalidRunLengthEncoding,
    InvalidValueType,
    RemainingData,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub fn read_u8(buffer: &[u8]) -> Result<(u8, &[u8])> {
    let [first, ref rest @ ..] = *buffer else {
        return Err(Error::EOF);
    };
    Ok((first, rest))
}

pub fn read_bool(buffer: &[u8]) -> Result<(bool, &[u8])> {
    let (byte, buffer) = read_u8(buffer)?;
    Ok((byte != 0, buffer))
}

pub fn read_u16(buffer: &[u8]) -> Result<(u16, &[u8])> {
    let [first, second, ref rest @ ..] = *buffer else {
        return Err(Error::EOF);
    };
    let value = u16::from_le_bytes([first, second]);
    Ok((value, rest))
}

pub fn read_i16_mut(buffer: &mut &[u8]) -> Result<i16> {
    let &[first, second, ref rest @ ..] = *buffer else {
        return Err(Error::EOF);
    };
    *buffer = rest;
    let value = i16::from_le_bytes([first, second]);
    Ok(value)
}

pub fn read_i16(buffer: &[u8]) -> Result<(i16, &[u8])> {
    let [first, second, ref rest @ ..] = *buffer else {
        return Err(Error::EOF);
    };
    let value = i16::from_le_bytes([first, second]);
    Ok((value, rest))
}

pub fn read_u32(buffer: &[u8]) -> Result<(u32, &[u8])> {
    let [first, second, third, fourth, ref rest @ ..] = *buffer else {
        return Err(Error::EOF);
    };
    let value = u32::from_le_bytes([first, second, third, fourth]);
    Ok((value, rest))
}

pub fn read_i32(buffer: &[u8]) -> Result<(i32, &[u8])> {
    let [first, second, third, fourth, ref rest @ ..] = *buffer else {
        return Err(Error::EOF);
    };
    let value = i32::from_le_bytes([first, second, third, fourth]);
    Ok((value, rest))
}

pub fn read_f32(buffer: &[u8]) -> Result<(f32, &[u8])> {
    let [first, second, third, fourth, ref rest @ ..] = *buffer else {
        return Err(Error::EOF);
    };
    let value = f32::from_le_bytes([first, second, third, fourth]);
    Ok((value, rest))
}

pub fn read_bytes(buffer: &[u8], n: usize) -> Result<(&[u8], &[u8])> {
    if n > buffer.len() {
        return Err(Error::EOF);
    }

    Ok(buffer.split_at(n))
}

pub fn read_run_length_encoded(buffer: &[u8]) -> Result<(String, &[u8])> {
    let (byte_count, buffer) = read_i16(buffer)?;
    let (data, buffer) = read_bytes(buffer, byte_count as usize)?;

    let part_len = byte_count
        .checked_div(2)
        .ok_or(Error::InvalidRunLengthEncoding)? as usize;
    let mut parts = vec![String::new(); part_len];

    for (part_index, i) in (0..byte_count).step_by(2).enumerate() {
        let [times, char, ..] = data[i as usize..] else {
            return Err(Error::EOF);
        };
        // TODO less allocation?
        parts[part_index] = String::from_utf8(vec![char; times as usize]).unwrap();
    }

    let string = parts.join("");

    Ok((string, buffer))
}

pub fn get_var_length(mut buffer: &[u8]) -> Result<(usize, &[u8])> {
    let mut res: usize = 0;
    let mut count = 0;
    loop {
        let byte;
        (byte, buffer) = read_u8(buffer)?;

        res += ((byte & 127) as usize) << (count * 7) as usize;
        count += 1;
        if byte >> 7 == 0 {
            return Ok((res, buffer));
        }
    }
}

pub fn read_byte_string(buffer: &[u8]) -> Result<(&[u8], &[u8])> {
    let (length, buffer) = get_var_length(buffer)?;
    let (string, buffer) = read_bytes(buffer, length)?;
    Ok((string, buffer))
}

pub fn read_string(buffer: &[u8]) -> Result<(&str, &[u8])> {
    let (string, buffer) = read_byte_string(buffer)?;
    let string = std::str::from_utf8(string).map_err(|_| Error::InvalidUTF8)?;
    Ok((string, buffer))
}

pub fn look<'a>(buffer: &'a [u8], lookup: &[&'a str]) -> Result<(&'a str, &'a [u8])> {
    let (index, buffer) = read_u16(buffer)?;
    let value = *lookup.get(index as usize).ok_or(Error::InvalidLookup)?;

    Ok((value, buffer))
}

pub fn decode_element<'a>(
    mut buffer: &'a [u8],
    lookup: &[&'a str],
) -> Result<(Element<'a>, &'a [u8])> {
    let name;
    (name, buffer) = look(buffer, lookup)?;

    let (attribute_count, mut buffer) = read_u8(buffer)?;
    let mut attributes = HashMap::with_capacity(attribute_count as usize);

    for _ in 0..attribute_count {
        let key;
        (key, buffer) = look(buffer, lookup)?;

        let ty;
        (ty, buffer) = read_u8(buffer)?;

        let value;
        (value, buffer) = decode_value(buffer, ty, lookup)?;
        attributes.insert(key, value);
    }

    let (child_count, mut buffer) = read_u16(buffer)?;
    let mut children = Vec::with_capacity(child_count as usize);

    for _ in 0..child_count {
        let child;
        (child, buffer) = decode_element(buffer, lookup)?;
        children.push(child);
    }

    let element = Element {
        name,
        attributes,
        children,
    };

    Ok((element, buffer))
}

pub fn decode_value<'a>(
    buffer: &'a [u8],
    ty: u8,
    lookup: &[&'a str],
) -> Result<(Value<'a>, &'a [u8])> {
    fn map_first<T, U, S>(f: impl Fn(T) -> U) -> impl Fn((T, S)) -> (U, S) {
        move |(val, second)| (f(val), second)
    }
    match ty {
        0 => read_bool(buffer).map(map_first(Value::Bool)),
        1 => read_u8(buffer).map(map_first(Value::U8)),
        2 => read_i16(buffer).map(map_first(Value::I16)),
        3 => read_i32(buffer).map(map_first(Value::I32)),
        4 => read_f32(buffer).map(map_first(Value::F32)),
        5 => look(buffer, lookup).map(map_first(|str| Value::String(Cow::Borrowed(str)))),
        6 => read_string(buffer).map(map_first(|str| Value::String(Cow::Borrowed(str)))),
        7 => read_run_length_encoded(buffer).map(map_first(|str| Value::String(Cow::Owned(str)))),
        _ => Err(Error::InvalidValueType),
    }
}
