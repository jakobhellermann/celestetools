use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fmt::Write,
    path::Path,
};

use anyhow::{Context, Result};
use celesteloader::{utils::list_dir_extension, CelesteInstallation};
use mlua::{AsChunk, Lua, Table, Value};

fn main() -> Result<()> {
    let celeste = CelesteInstallation::detect()?;
    let lua_plugins: Vec<_> = celeste
        .mods_with(|_path, mut archive| {
            let files: Vec<_> = archive
                .list_files()
                .filter(|file| file.starts_with("Loenn/entities") && file.ends_with("lua"))
                .map(ToOwned::to_owned)
                .collect();

            let lua_plugins = files
                .into_iter()
                .map(|file| {
                    let x = archive.read_file_string(&file)?;
                    Ok((_path.to_owned(), file, x))
                })
                .collect::<Result<Vec<_>, anyhow::Error>>()?;

            Ok(lua_plugins)
        })?
        .into_iter()
        .flatten()
        .collect();

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
function fakeTiles.getEntitySpriteFunction(a, b) return nil end
function fakeTiles.getFieldInformation(a) return nil end
package.preload["helpers.fake_tiles"] = function() return fakeTiles end


"#,
    )
    .exec()?;
    lua.load(include_str!("./bit.lua")).exec()?;

    let mut results = BTreeMap::new();

    let from_celeste = true;
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

    if from_celeste {
        let loenn_src = Path::new("/home/jakob/dev/celeste/Loenn/src/");
        list_dir_extension(&loenn_src.join("entities"), "lua", |path| -> Result<()> {
            load_entity_plugin(
                &lua,
                path.display().to_string(),
                path,
                &mut stats,
                &mut results,
            )?;
            Ok(())
        })?;
    }

    let mut errors = stats.errors.iter().map(|(k, v)| (v, k)).collect::<Vec<_>>();
    errors.sort_by_key(|&(i, _str)| std::cmp::Reverse(i.len()));
    /*for (v, k) in errors.iter().take(20) {
        eprintln!("{:3}: {} ({:?})", v.len(), k, &format!("{:?}", v)[..100]);
    }*/

    let mut out = String::new();

    writeln!(
        &mut out,
        r"use super::RenderMethod;
use std::collections::HashMap;

#[rustfmt::skip]
pub fn render_methods() -> HashMap<&'static str, RenderMethod> {{
    let mut textures = HashMap::new();
"
    )?;

    let blacklist = HashSet::<_>::from_iter(["oshirodoor"]);

    for (name, render) in results {
        if blacklist.contains(&name.as_str()) {
            continue;
        }

        match render {
            EntityRender::Texture(texture, justification) => {
                writeln!(
                    &mut out,
                    r#"    textures.insert("{name}", RenderMethod::Texture {{ texture: "{texture}", justification: {justification:?} }});"#
                )?;
            }
            EntityRender::Rect(fill, border) => {
                writeln!(
                    &mut out,
                    r#"    textures.insert("{name}", RenderMethod::Rect {{ fill: {fill:?}, border: {border:?} }});"#
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
    stats_error: u32,
    stats_nil: u32,
    stats_texture_str: u32,
    stats_texture_func: u32,
    stats_texture_other: u32,
    no_texture: u32,
}

impl std::fmt::Debug for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stats")
            .field("stats_error", &self.stats_error)
            .field("stats_nil", &self.stats_nil)
            .field("stats_texture_str", &self.stats_texture_str)
            .field("stats_texture_other", &self.stats_texture_other)
            .field("no_texture", &self.no_texture)
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
            stats.stats_error += 1;

            let msg = error.to_string();
            let msg = msg
                .lines()
                .next()
                .unwrap()
                .trim_end_matches(":")
                .rsplit(":")
                .next()
                .unwrap()
                .trim();
            stats.errors.entry(msg.to_string()).or_default().push(file);
            return Ok(());

            /*eprintln!(
                "{}",
                /*_e.to_string()
                .lines()
                .next()
                .unwrap()
                .split(":")
                .nth(3)
                .unwrap(),*/
                // _e.to_string().lines().next().unwrap()
            );*/
        }
        Ok(val) => val,
    };

    if val.is_nil() {
        stats.stats_nil += 1;
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

fn table_to_color(table: Table) -> (u8, u8, u8, u8) {
    let mut vals = table.sequence_values::<f32>();

    let r = vals.next().unwrap().unwrap();
    let g = vals.next().unwrap().unwrap();
    let b = vals.next().unwrap().unwrap();
    let a = vals.next().unwrap_or(Ok(1.0)).unwrap();

    (
        (r * 256.) as u8,
        (g * 256.) as u8,
        (b * 256.) as u8,
        (a * 256.) as u8,
    )
}

enum EntityRender {
    Texture(String, Option<(f32, f32)>),
    Rect(Option<(u8, u8, u8, u8)>, Option<(u8, u8, u8, u8)>),
}

fn extract_value(
    lua: &Lua,
    table: &Table,
    _file: &str,
    stats: &mut Stats,
    results: &mut BTreeMap<String, EntityRender>,
) -> Result<()> {
    let name = table.get::<_, String>("name").unwrap();

    let fill_color = table.get::<_, Option<Value>>("fillColor")?;
    let border_color = table.get::<_, Option<Value>>("borderColor")?;

    if fill_color.is_some() || border_color.is_some() {
        let fill_color = fill_color.and_then(|color| {
            match color {
                Value::Function(_) => return None, // TODO
                Value::Table(table) => Some(table_to_color(table)),
                _ => unimplemented!(),
            }
        });
        let border_color = border_color.and_then(|color| {
            match color {
                Value::Function(_) => return None, // TODO
                Value::Table(table) => Some(table_to_color(table)),
                _ => unimplemented!(),
            }
        });

        results.insert(name, EntityRender::Rect(fill_color, border_color));

        return Ok(());
    }

    if !table.contains_key("texture").context("a")? {
        stats.no_texture += 1;
        return Ok(());
    }

    let justification = match table.get::<_, Value>("justification")? {
        Value::Nil => None,
        Value::Table(table) => {
            let x = table.get::<_, f32>(1)?;
            let y = table.get::<_, f32>(2)?;
            Some((x, y))
        }
        Value::Function(_) => None,
        _ => todo!(),
    };

    let texture = table.get::<_, Value>("texture").context("b")?;

    match texture {
        Value::String(str) => {
            stats.stats_texture_str += 1;
            results.insert(
                name,
                EntityRender::Texture(str.to_string_lossy().into(), justification),
            );
        }
        Value::Function(func) => {
            stats.stats_texture_func += 1;

            unsafe extern "C-unwind" fn index(lua: *mut mlua::lua_State) -> i32 {
                let _ = lua;
                0
            }

            let entity = lua.create_table()?;
            let metatable = lua.create_table()?;
            let index = unsafe { lua.create_c_function(index)? };
            metatable.set("__index", index)?;
            entity.set_metatable(Some(metatable));

            match func.call::<_, mlua::Value>((mlua::Nil, mlua::Value::Table(entity))) {
                Ok(_) => {}
                Err(_) => {
                    // eprintln!("{file} {e}")
                }
            }

            return Ok(());
        }
        _ => stats.stats_texture_other += 1,
    }

    Ok(())
}
