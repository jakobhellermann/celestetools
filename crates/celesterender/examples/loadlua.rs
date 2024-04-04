use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fmt::Write,
    fs::File,
    io::BufReader,
    path::Path,
};

use anyhow::{Context, Result};
use celesteloader::{archive::ModArchive, utils::list_dir_extension};
use mlua::{AsChunk, FromLua, Function, Lua, MultiValue, Table, Value};

fn main() -> Result<()> {
    let downloaded_mods: Vec<(String, File)> = Path::new("downloads")
        .read_dir()
        .unwrap()
        .map(|x| {
            let path = x.unwrap().path();
            (path.display().to_string(), File::open(path).unwrap())
        })
        .collect();

    let mut lua_plugins = Vec::new();
    for (path, file) in downloaded_mods {
        let mut archive = ModArchive::new(BufReader::new(file)).context(path.clone())?;

        let files: Vec<_> = archive
            .list_files()
            .filter(|file| file.starts_with("Loenn/entities") && file.ends_with("lua"))
            .map(ToOwned::to_owned)
            .collect();

        let plugins = files
            .into_iter()
            .map(|file| {
                let x = archive.read_file_string(&file)?;
                Ok((path.to_owned(), file, x))
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()?;
        lua_plugins.extend(plugins);
    }

    /* let lua_plugins: Vec<_> = celeste
    .mods_with(|_path, mut archive| {

        Ok(lua_plugins)
    })?
    .into_iter()
    .flatten()
    .collect()*/

    let mut stats = Stats::default();

    let lua = unsafe { Lua::unsafe_new() };
    lua.load(
        r#"
_G.load = function()
    print("load called")
end

unpack = table.unpack

package.preload["ffi"] = function() return { os = "Linux" } end

love = {}

package.path = ''

--package.path = '/home/jakob/dev/celeste/Loenn/src/?.lua;/home/jakob/dev/celeste/Loenn/src/?/?.lua;/home/jakob/dev/celeste/Loenn/src/selene/selene/lib/?/init.lua;/home/jakob/dev/celeste/Loenn/src/selene/selene/lib/?.lua;'-- .. package.path;
--require("selene").load(nil, true)

--package.path = '/home/jakob/dev/celeste/Loenn/src/selene/selene/lib/?/init.lua;/home/jakob/dev/celeste/Loenn/src/selene/selene/lib/?.lua;
--require("selene").load(nil, true)



drawableSprite = {}
drawableSpriteMt = {}
drawableSpriteMt.__index = {}
function drawableSpriteMt.__index.fromTexture(texture, entity) end
setmetatable(drawableSprite, drawableSpriteMt)
package.preload["structs.drawable_sprite"] = function() return drawableSprite end

package.preload["structs.drawable_function"] = function() return nil end
package.preload["structs.drawable_line"] = function() return nil end
package.preload["structs.drawable_rectangle"] = function() return nil end
package.preload["structs.drawable_nine_patch"] = function() return nil end
package.preload["utils.drawing"] = function() return nil end
package.preload["utils.matrix"] = function() return nil end
package.preload["entities"] = function() return nil end
package.preload["helpers.spikes"] = function() return nil end
package.preload["helpers.waterfalls"] = function() return nil end
package.preload["helpers.flagline"] = function() return nil end
package.preload["atlases"] = function() return nil end


mods = {}
function mods.requireFromPlugin(plugin) return {} end
package.preload["mods"] = function() return mods end

package.preload["utils"] = function() return {} end
package.preload["consts.celeste_enums"] = function() return {} end
package.preload["consts.xna_colors"] = function() return {} end
package.preload["helpers.connected_entities"] = function() return {} end
package.preload["helpers.resort_platforms"] = function() return {} end


fakeTiles = {}
function fakeTiles.getEntitySpriteFunction(materialKey, blendKey, layer, color, x, y)
    res={ materialKey, blendKey, layer, color, x, y }
    res.fakeTile = true
    return res
end
function fakeTiles.getFieldInformation(a) return nil end

package.preload["helpers.fake_tiles"] = function()
    return fakeTiles
end


"#,
    )
    .exec()?;
    lua.load(include_str!("./bit.lua")).exec()?;

    let mut results = BTreeMap::new();

    let from_vanilla = true;
    let from_mods = true;

    if from_mods {
        for (map, name, plugin) in lua_plugins {
            load_entity_plugin(
                &lua,
                format!("{map}:{name}"),
                plugin,
                &mut stats,
                &mut results,
            )
            .context(name)
            .context(map)?;
        }
    }

    if from_vanilla {
        let loenn_src = Path::new("../Loenn/src/");
        list_dir_extension(&loenn_src.join("entities"), "lua", |path| -> Result<()> {
            let file = path.display().to_string();

            load_entity_plugin(&lua, file, path, &mut stats, &mut results)?;
            Ok(())
        })?;
    }

    let mut errors = stats.errors.iter().map(|(k, v)| (v, k)).collect::<Vec<_>>();
    errors.sort_by_key(|&(i, _str)| std::cmp::Reverse(i.len()));
    /*for (v, k) in errors.iter().take(20) {
        eprintln!("{:3}: {} ({:?})", v.len(), k, &format!("{:?}", v)[..100]);
    }*/

    dbg!(stats);
    dbg!(results.len());

    let mut out = String::new();

    writeln!(
        &mut out,
        r"use super::RenderMethod;
use tiny_skia::Color;
use std::collections::HashMap;

#[rustfmt::skip]
pub fn render_methods() -> HashMap<&'static str, RenderMethod> {{
    let mut textures = HashMap::new();
"
    )?;

    let blacklist = HashSet::<_>::from_iter([
        "oshirodoor",
        "MaxHelpingHand/OneWayInvisibleBarrierHorizontal",
    ]);

    for (name, render) in results {
        if blacklist.contains(&name.as_str()) {
            continue;
        }

        match render {
            EntityRender::Texture(texture, justification, rotation) => {
                writeln!(
                    &mut out,
                    r#"    textures.insert("{name}", RenderMethod::Texture {{ texture: "{texture}", justification: {justification:?}, rotation: {rotation:?} }});"#
                )?;
            }
            EntityRender::Rect(fill, border) => {
                writeln!(
                    &mut out,
                    r#"    textures.insert("{name}", RenderMethod::Rect {{ fill: {fill:?}, border: {border:?} }});"#
                )?;
            }
            EntityRender::FakeTiles {
                material_key,
                blend_key,
                layer,
                color,
                x,
                y,
            } => {
                writeln!(
                    &mut out,
                    r#"    textures.insert("{name}", RenderMethod::FakeTiles {{
        material_key: {material_key:?},
        blend_key: {blend_key:?},
        layer: {layer:?},
        color: {color:?},
        x: {x:?},
        y: {y:?},
    }});"#
                )?;
            }
        }
    }
    writeln!(&mut out, "\n    textures\n}}")?;

    std::fs::write(
        "crates/celesterender/src/rendering/entity/entity_impls.rs",
        out,
    )?;

    Ok(())
}

#[derive(Default)]
struct Stats {
    errors: HashMap<String, Vec<String>>,
    error: u32,
    nil: u32,
    sprite_fn: u32,
    draw_fn: u32,
    other: u32,
    fn_call_error: u32,
    fn_call_noresult: u32,
    fn_call_nil: u32,
    fn_call_ok: u32,
}

impl std::fmt::Debug for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stats")
            .field("error", &self.error)
            .field("nil", &self.nil)
            .field("sprite_fn", &self.sprite_fn)
            .field("draw_fn", &self.draw_fn)
            .field("stats_other", &self.other)
            .field("fn_call_error", &self.fn_call_error)
            .field("fn_call_noresult", &self.fn_call_noresult)
            .field("fn_call_nil", &self.fn_call_nil)
            .field("fn_call_ok", &self.fn_call_ok)
            .finish()
    }
}

fn load_entity_plugin<'lua, 'a>(
    lua: &'lua Lua,
    file: String,
    chunk: impl AsChunk<'lua, 'a>,
    stats: &mut Stats,
    results: &mut BTreeMap<String, EntityRender>,
) -> Result<()> {
    let chunk = lua.load(chunk);

    let val = match chunk.eval::<Value>() {
        Err(error) => {
            stats.error += 1;

            let msg = error.to_string();
            let msg = msg
                .lines()
                .next()
                .unwrap()
                .trim_end_matches(':')
                .rsplit(':')
                .next()
                .unwrap()
                .trim();
            stats.errors.entry(msg.to_string()).or_default().push(file);
            return Ok(());
        }
        Ok(val) => val,
    };

    if val.is_nil() {
        stats.nil += 1;
        return Ok(());
    }

    let table = val.as_table().context("not a table")?;

    if !table.contains_key("name")? {
        let mut errors = Vec::new();
        table
            .for_each(|_: u32, v: Table| {
                if let Err(e) = extract_value(lua, &v, &file, stats, results) {
                    errors.push(e);
                }

                Ok(())
            })
            .unwrap();

        for error in errors {
            return Err(error);
        }

        return Ok(());
    }

    extract_value(lua, table, &file, stats, results)?;

    Ok(())
}

enum EntityRender {
    Texture(String, Option<(f32, f32)>, Option<f32>),
    Rect(Color, Color),
    FakeTiles {
        material_key: String,
        blend_key: bool,
        layer: Option<String>,
        color: Option<Color>,
        x: Option<String>,
        y: Option<String>,
    },
}

#[derive(Clone, Copy)]
struct Color([f32; 4]);
impl<'lua> FromLua<'lua> for Color {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> mlua::prelude::LuaResult<Self> {
        let vals = Table::from_lua(value, lua)?;
        let mut vals = vals.sequence_values();
        let r = vals.next().unwrap().unwrap();
        let g = vals.next().unwrap().unwrap();
        let b = vals.next().unwrap().unwrap();
        let a = vals.next().unwrap_or(Ok(1.0)).unwrap();

        Ok(Color([r, g, b, a]))
    }
}
impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Color::from_rgba8({}, {}, {}, {})",
            (self.0[0] * 256.) as u8,
            (self.0[1] * 256.) as u8,
            (self.0[2] * 256.) as u8,
            (self.0[3] * 256.) as u8
        )
    }
}

fn from_lua_or_function<'lua, T: FromLua<'lua>>(
    lua: &'lua Lua,
    val: Value<'lua>,
    stats: &mut Stats,
) -> Result<Option<T>> {
    match val {
        Value::Function(func) => {
            let entity = lua.create_table()?;
            let metatable = lua.create_table()?;
            let index = lua.create_function(|_lua, val: MultiValue| {
                let mut args = val.into_iter();
                let _first = args.next().unwrap();
                let _second = args.next().unwrap();
                // eprintln!("got queried {second:?}");
                Ok(Value::Nil)
            })?;
            metatable.set("__index", index)?;
            entity.set_metatable(Some(metatable));

            let result =
                match func.call::<_, mlua::MultiValue>((mlua::Nil, mlua::Value::Table(entity))) {
                    Ok(val) => val,
                    Err(_) => {
                        stats.fn_call_error += 1;
                        return Ok(None);
                    }
                };

            match result.len() {
                0 => {
                    stats.fn_call_noresult += 1;
                    Ok(None)
                }
                1 => match result.into_iter().next().unwrap() {
                    Value::Nil => {
                        stats.fn_call_nil += 1;
                        Ok(None)
                    }
                    val => {
                        let val = T::from_lua(val, lua)?;

                        stats.fn_call_ok += 1;
                        Ok(Some(val))
                    }
                },
                _ => {
                    let table = lua.create_table()?;
                    for val in result.into_iter() {
                        table.push(val)?;
                    }

                    let x = T::from_lua(Value::Table(table), lua).unwrap();
                    Ok(Some(x))
                }
            }
        }
        Value::Nil => Ok(None),
        val => {
            let val = T::from_lua(val, lua)?;
            Ok(Some(val))
        }
    }
}

fn extract_value(
    lua: &Lua,
    table: &Table,
    _file: &str,
    stats: &mut Stats,
    results: &mut BTreeMap<String, EntityRender>,
) -> Result<()> {
    let name = table.get::<_, String>("name").unwrap();

    let has_rectangle = table.get::<_, Option<Function>>("rectangle")?.is_some();

    let color = from_lua_or_function::<Color>(lua, table.get::<_, Value>("color")?, stats)?;
    let fill = from_lua_or_function::<Color>(lua, table.get::<_, Value>("fillColor")?, stats)?;
    let border = from_lua_or_function::<Color>(lua, table.get::<_, Value>("borderColor")?, stats)?;

    if !has_rectangle {
        if let (Some(fill), Some(border)) = (fill, border) {
            results.insert(name.clone(), EntityRender::Rect(fill, border));
            return Ok(());
        }
    }

    let justification = from_lua_or_function::<[f32; 2]>(lua, table.get("justification")?, stats)?;
    let justification = justification.map(|[x, y]| (x, y));

    let texture = from_lua_or_function::<String>(lua, table.get::<_, Value>("texture")?, stats)?;

    if let Some(texture) = texture {
        let rotation = from_lua_or_function::<f32>(lua, table.get("rotation")?, stats)?;

        results.insert(
            name.clone(),
            EntityRender::Texture(texture, justification, rotation),
        );

        if color.is_some() {
            println!("todo: tint {}", name);
        }
        return Ok(());
    };

    if !has_rectangle {
        if let Some(color) = color {
            results.insert(name, EntityRender::Rect(color, color));
            return Ok(());
        }
    }

    match table.get::<_, Value>("sprite")? {
        Value::Nil => {}
        Value::Function(_) => {
            stats.sprite_fn += 1;
            return Ok(());
        }
        Value::Table(table) if table.get::<_, bool>("fakeTile")? => {
            let material_key = table.get::<_, String>(1)?;
            let blend_key = match table.get::<_, Value>(2)? {
                Value::Nil => false,
                Value::String(str) if str == "blendin" => true,
                Value::Boolean(val) => val,
                other => unimplemented!("{other:?}"),
            };
            let layer = table.get::<_, Option<String>>(3).context("layer")?;
            let color = table.get::<_, Option<Color>>(4).context("color")?;
            let x = table.get::<_, Option<String>>(5).context("x")?;
            let y = table.get::<_, Option<String>>(6).context("y")?;

            results.insert(
                name,
                EntityRender::FakeTiles {
                    material_key,
                    blend_key,
                    layer,
                    color,
                    x,
                    y,
                },
            );
            return Ok(());
        }
        _ => unimplemented!(),
    };

    match table.get::<_, Value>("draw")? {
        Value::Nil => {}
        Value::Function(_) => {
            stats.draw_fn += 1;
            return Ok(());
        }
        _ => unimplemented!(),
    };

    stats.other += 1;
    Ok(())
}
