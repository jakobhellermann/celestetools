use std::path::PathBuf;

use crate::binaryreader::*;

#[derive(Debug)]
pub struct AtlasMeta {
    pub header: String,
    pub data: PathBuf,
    pub sprites: Vec<Sprite>,
}

#[derive(Debug)]
pub struct Sprite {
    pub path: String,
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
    pub offset_x: i16,
    pub offset_y: i16,
    pub real_w: i16,
    pub real_h: i16,
}

pub fn decode_atlas(buffer: &[u8]) -> Result<Vec<AtlasMeta>> {
    let (_, buffer) = read_i32(buffer)?;
    let (header, buffer) = read_string(buffer)?;
    let (_, buffer) = read_i32(buffer)?;

    let (count, mut buffer) = read_u16(buffer)?;

    let mut banks = Vec::with_capacity(count as usize);

    for _ in 0..banks.capacity() {
        let data_file;
        let n_sprites;
        (data_file, buffer) = read_string(buffer)?;
        (n_sprites, buffer) = read_i16(buffer)?;

        let mut sprites = Vec::with_capacity(n_sprites as usize);
        for _ in 0..n_sprites {
            let path_raw;
            (path_raw, buffer) = read_string(buffer)?;
            let path = path_raw.replace("\\", "/");

            let x = read_i16_mut(&mut buffer)?;
            let y = read_i16_mut(&mut buffer)?;
            let w = read_i16_mut(&mut buffer)?;
            let h = read_i16_mut(&mut buffer)?;
            let offset_x = read_i16_mut(&mut buffer)?;
            let offset_y = read_i16_mut(&mut buffer)?;
            let real_w = read_i16_mut(&mut buffer)?;
            let real_h = read_i16_mut(&mut buffer)?;

            sprites.push(Sprite {
                path,
                x,
                y,
                w,
                h,
                offset_x,
                offset_y,
                real_w,
                real_h,
            })
        }

        banks.push(AtlasMeta {
            header: header.to_owned(),
            data: PathBuf::from(data_file),
            sprites,
        });
    }

    Ok(banks)
}

pub fn decode_data(data: &[u8]) -> Result<image::RgbaImage, Error> {
    let (width, data) = read_u32(data)?;
    let (height, data) = read_u32(data)?;
    let (has_alpha, mut data) = read_bool(data)?;

    let mut buf: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);

    while let Some((&run_length, remaining)) = data.split_first() {
        data = remaining;

        let color = if has_alpha {
            let a;
            (a, data) = data.split_first().unwrap();

            let [b, g, r] = match a {
                0 => [0, 0, 0],
                _ => {
                    let color;
                    (color, data) = data.split_at(3);
                    color.try_into().unwrap()
                }
            };

            [r, g, b, *a]
        } else {
            let color;
            (color, data) = data.split_at(3);
            let [b, g, r] = color.try_into().unwrap();
            [r, g, b, 255]
        };

        for _ in 0..run_length {
            buf.extend(color);
        }
    }

    let image = image::RgbaImage::from_raw(width, height, buf).unwrap();
    Ok(image)
}
