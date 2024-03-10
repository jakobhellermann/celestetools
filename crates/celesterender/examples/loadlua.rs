use std::{collections::HashMap, path::Path};

use anyhow::{Context, Result};
use celesteloader::{utils::list_dir_extension, CelesteInstallation};
use mlua::{AsChunk, Lua, Value};

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

    let mut results = Vec::new();

    if false {
        for (map, name, plugin) in lua_plugins {
            if let Some(x) = load_entity_plugin(&lua, format!("{map}:{name}"), plugin, &mut stats)
                .context(name)
                .context(map)?
            {
                results.push(x);
            }
        }
    }

    if true {
        let loenn_src = Path::new("/home/jakob/dev/celeste/Loenn/src/");
        list_dir_extension(&loenn_src.join("entities"), "lua", |path| -> Result<()> {
            if let Some(x) = load_entity_plugin(&lua, path.display().to_string(), path, &mut stats)?
            {
                results.push(x);
            }
            Ok(())
        })?;
    }

    let mut errors = stats.errors.iter().map(|(k, v)| (v, k)).collect::<Vec<_>>();
    errors.sort_by_key(|&(i, _str)| std::cmp::Reverse(i.len()));
    /*for (v, k) in errors.iter().take(20) {
        eprintln!("{:3}: {} ({:?})", v.len(), k, &format!("{:?}", v)[..100]);
    }*/

    // dbg!(results);
    dbg!(stats);

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
) -> Result<Option<String>> {
    let chunk = lua.load(chunk);

    match chunk.eval::<Value>() {
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
        Ok(val) => {
            if val.is_nil() {
                stats.stats_nil += 1;
                return Ok(None);
            }
            let table = val.as_table().context("not a table")?;
            if table.contains_key("texture").context("a")? {
                let texture = table.get::<_, Value>("texture").context("b")?;

                match texture {
                    Value::String(str) => {
                        stats.stats_texture_str += 1;
                        return Ok(Some(str.to_string_lossy().into()));
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
                            Err(e) => {
                                eprintln!("{file} {e}")
                            }
                        }

                        return Ok(None);
                    }
                    _ => stats.stats_texture_other += 1,
                }
            } else {
                stats.no_texture += 1;
            }
        }
    }

    Ok(None)
}
