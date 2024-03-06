use std::borrow::Cow;

use crate::binaryreader::*;
pub use crate::binaryreader::{Element, Error, Value, ValueType};

pub fn decode_map(buffer: &[u8]) -> Result<Element<'_>> {
    let (header, buffer) = read_byte_string(buffer)?;
    if header != b"CELESTE MAP" {
        return Err(Error::InvalidHeader);
    }

    let (package, buffer) = read_string(buffer)?;
    let (lookup_length, mut buffer) = read_i16(buffer)?;

    let mut lookup = Vec::with_capacity(lookup_length as usize);
    for _ in 0..lookup_length {
        let entry;
        (entry, buffer) = read_string(buffer)?;
        lookup.push(entry);
    }

    let (mut res, buffer) = decode_element(buffer, &lookup)?;
    res.attributes
        .insert("package", Value::String(Cow::Borrowed(package)));

    // prologue has a trailing '10' for some reason
    if buffer.len() > 1 {
        return Err(Error::RemainingData);
    }

    Ok(res)
}
