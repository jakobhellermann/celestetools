use std::collections::HashMap;

use anyhow::{anyhow, Result};
use celesteloader::tileset::Tileset;

#[derive(Clone)]
pub struct ParsedTileset {
    pub path: String,
    ignores: Ignores,
    set: Vec<MaskData>,
}

#[derive(Clone, Debug)]
pub enum Ignores {
    All,
    None,
    List(Vec<char>),
}
impl Ignores {
    pub fn ignores(&self, c: char) -> bool {
        match self {
            Ignores::All => true,
            Ignores::None => false,
            Ignores::List(list) => list.contains(&c),
        }
    }
}

impl ParsedTileset {
    pub fn parse(tilesets: &[Tileset]) -> Result<HashMap<char, ParsedTileset>> {
        let mut built = HashMap::<char, ParsedTileset>::with_capacity(tilesets.len());
        for tileset in tilesets {
            let mut rules = match tileset.copy {
                Some(copy) => built.get(&copy).unwrap().set.clone(),
                _ => Vec::with_capacity(tileset.set.len()),
            };

            for set in &tileset.set {
                let mask = parse_mask_string(&set.mask)
                    .ok_or_else(|| anyhow!("failed to parse tileset mask '{}'", set.mask))?;
                let tiles = parse_set_tiles(&set.tiles)
                    .ok_or_else(|| anyhow!("failed to parse tileset tiles '{}'", set.tiles))?;

                rules.push(MaskData { mask, tiles });
            }

            let ignores = tileset
                .ignores
                .as_ref()
                .map(|ignores| -> Result<_> {
                    let ignores = ignores.trim();

                    if ignores.is_empty() {
                        Ok(Ignores::None)
                    } else if ignores == "*" {
                        Ok(Ignores::All)
                    } else {
                        let list = ignores
                            .split(',')
                            .map(|x| {
                                if x.chars().count() != 1 {
                                    return None;
                                }

                                Some(x.chars().next().unwrap())
                            })
                            .collect::<Option<Vec<_>>>()
                            .ok_or_else(|| anyhow!("failed to parse ignores '{ignores}'"))?;

                        Ok(Ignores::List(list))
                    }
                })
                .transpose()?
                .unwrap_or(Ignores::None);

            // TODO sort

            built.insert(
                tileset.id,
                ParsedTileset {
                    path: tileset.path.clone(),
                    ignores: ignores,
                    set: rules,
                },
            );
        }
        Ok(built)
    }
}

pub fn tiles_to_matrix_scenery(tile_size: (u32, u32), tiles: &str) -> Matrix<i16> {
    let mut backing = Vec::with_capacity((tile_size.0 * tile_size.1) as usize);

    let mut i = 0;
    for line in tiles.lines() {
        let before = backing.len();
        if !line.is_empty() {
            backing.extend(line.split(',').map(|val| val.parse::<i16>().unwrap()));
        }
        let after = backing.len();

        let remaining = tile_size.0 as usize - (after - before);
        for _ in 0..remaining {
            backing.push(-1);
        }

        assert_eq!((after - before) + remaining, tile_size.0 as usize);

        i += 1;
    }
    let remaining_lines = tile_size.1 as usize - i;

    for _ in 0..remaining_lines {
        for _ in 0..tile_size.0 {
            backing.push(-1);
        }
    }

    assert_eq!(backing.len(), (tile_size.0 * tile_size.1) as usize);

    Matrix {
        size: tile_size,
        backing,
    }
}

pub(crate) const AIR: char = '0';

pub fn tiles_to_matrix(tile_size: (u32, u32), tiles: &str) -> Result<Matrix<char>> {
    let mut backing = Vec::with_capacity((tile_size.0 * tile_size.1) as usize);

    let mut i = 0;
    for line in tiles.lines() {
        let before = backing.len();
        backing.extend(line.chars());
        let after = backing.len();
        let added = after - before;

        let remaining = (tile_size.0 as isize) - added as isize; // lvl_resort-credits says hello
        backing.resize((backing.len() as isize + remaining) as usize, AIR);

        assert_eq!(added as isize + remaining, tile_size.0 as isize);

        i += 1;
    }
    let remaining_lines = tile_size.1 as usize - i;
    backing.resize(backing.len() + tile_size.0 as usize * remaining_lines, AIR);

    assert_eq!(backing.len(), (tile_size.0 * tile_size.1) as usize);

    Ok(Matrix {
        size: tile_size,
        backing,
    })
}

pub(crate) struct Matrix<T> {
    size: (u32, u32),
    backing: Vec<T>,
}

impl<T: Copy> Matrix<T> {
    pub fn from_fn(width: u32, height: u32, f: impl Fn(u32, u32) -> T + Copy) -> Self {
        Matrix {
            size: (width, height),
            backing: (0..height)
                .flat_map(|y| (0..width).map(move |x| f(x, y)))
                .collect(),
        }
    }
    pub(crate) fn get(&self, x: u32, y: u32) -> T {
        assert!(x < self.size.0);
        let idx = self.size.0 * y + x;
        self.backing[idx as usize]
    }
    pub(crate) fn get_or(&self, x: i32, y: i32, default: T) -> T {
        if x >= self.size.0 as i32 || x < 0 {
            return default;
        }
        if y >= self.size.1 as i32 || y < 0 {
            return default;
        }

        let idx = self.size.0 * y as u32 + x as u32;
        self.backing.get(idx as usize).copied().unwrap_or(default)
    }
}

#[derive(Clone)]
struct MaskData {
    mask: AutotilerMask,
    tiles: Vec<(u8, u8)>,
}

#[derive(Debug, Clone)]
enum AutotilerMask {
    Padding,
    Center,
    Pattern([[AutotilerMaskSegment; 3]; 3]),
}

#[derive(Debug, Clone, Copy)]
enum AutotilerMaskSegment {
    Present,
    Absent,
    Wildcard,
}
impl AutotilerMaskSegment {
    fn matches(&self, center: char, neighbor: char, ignores: &Ignores) -> bool {
        match self {
            AutotilerMaskSegment::Present => neighbor != AIR,
            // AutotilerMaskSegment::Present => neighbor != AIR,
            AutotilerMaskSegment::Absent => {
                neighbor == AIR || (neighbor != center && ignores.ignores(neighbor))
            }
            AutotilerMaskSegment::Wildcard => true,
        }
    }
}
fn parse_mask_string(str: &str) -> Option<AutotilerMask> {
    match str {
        "padding" => return Some(AutotilerMask::Padding),
        "center" => return Some(AutotilerMask::Center),
        _ => {}
    }

    let values: Vec<_> = str.split('-').collect();
    let [a, b, c] = values.as_slice() else {
        // eprintln!("warning: non-3x3 autotiler mask");

        return Some(AutotilerMask::Pattern([
            [
                AutotilerMaskSegment::Wildcard,
                AutotilerMaskSegment::Wildcard,
                AutotilerMaskSegment::Wildcard,
            ],
            [
                AutotilerMaskSegment::Wildcard,
                AutotilerMaskSegment::Wildcard,
                AutotilerMaskSegment::Wildcard,
            ],
            [
                AutotilerMaskSegment::Wildcard,
                AutotilerMaskSegment::Wildcard,
                AutotilerMaskSegment::Wildcard,
            ],
        ]));
    };

    let mask_from_val = |val: u8| match val {
        b'1' => AutotilerMaskSegment::Present,
        b'0' => AutotilerMaskSegment::Absent,
        b'x' => AutotilerMaskSegment::Wildcard,
        _ => unimplemented!("{}", char::from(val)),
    };
    let parse_row = |a: &str| -> [AutotilerMaskSegment; 3] {
        let row = a.bytes().map(mask_from_val).collect::<Vec<_>>();

        if let Ok(val) = row.try_into() {
            val
        } else {
            // eprintln!("warning: non-3x3 autotiler mask");
            [
                AutotilerMaskSegment::Wildcard,
                AutotilerMaskSegment::Wildcard,
                AutotilerMaskSegment::Wildcard,
            ]
        }
    };

    Some(AutotilerMask::Pattern([
        parse_row(a),
        parse_row(b),
        parse_row(c),
    ]))
}

fn parse_set_tiles(str: &str) -> Option<Vec<(u8, u8)>> {
    str.split(';')
        .map(|val| {
            let (x, y) = val.trim().split_once(',')?;
            let x = x.parse().ok()?;
            let y = y.parse().ok()?;
            Some((x, y))
        })
        .collect()
}

impl AutotilerMask {
    fn validate(&self, x: u32, y: u32, matrix: &Matrix<char>, ignores: &Ignores) -> bool {
        let center = matrix.get(x, y);
        match self {
            AutotilerMask::Padding => {
                let left = matrix.get_or(x as i32 - 2, y as i32, center);
                let right = matrix.get_or(x as i32 + 2, y as i32, center);
                let up = matrix.get_or(x as i32, y as i32 - 2, center);
                let down = matrix.get_or(x as i32, y as i32 + 2, center);

                let is_air = |x| x == AIR || (x != center && ignores.ignores(x));
                is_air(left) || is_air(right) || is_air(up) || is_air(down)
            }
            #[rustfmt::skip]
            #[allow(clippy::identity_op)]
            AutotilerMask::Pattern(pattern) => {
                       pattern[0][0].matches(center, matrix.get_or(x as i32  - 1, y as i32 - 1, center), ignores)
                    && pattern[0][1].matches(center, matrix.get_or(x as i32  + 0, y as i32 - 1, center), ignores)
                    && pattern[0][2].matches(center, matrix.get_or(x as i32  + 1, y as i32 - 1, center), ignores)
                    && pattern[1][0].matches(center, matrix.get_or(x as i32  - 1, y as i32 + 0, center), ignores)
                    && pattern[1][1].matches(center, matrix.get_or(x as i32  + 0, y as i32 + 0, center), ignores)
                    && pattern[1][2].matches(center, matrix.get_or(x as i32  + 1, y as i32 + 0, center), ignores)
                    && pattern[2][0].matches(center, matrix.get_or(x as i32  - 1, y as i32 + 1, center), ignores)
                    && pattern[2][1].matches(center, matrix.get_or(x as i32  + 0, y as i32 + 1, center), ignores)
                    && pattern[2][2].matches(center, matrix.get_or(x as i32  + 1, y as i32 + 1, center), ignores)
            },
            AutotilerMask::Center => true,
        }
    }
}

pub fn choose_tile<'a>(
    tileset: &'a ParsedTileset,
    x: u32,
    y: u32,
    tiles: &Matrix<char>,
) -> Result<Option<&'a [(u8, u8)]>> {
    for set in &tileset.set {
        if set.mask.validate(x, y, tiles, &tileset.ignores) {
            return Ok(Some(&set.tiles));
        }
    }

    Ok(None)
}
