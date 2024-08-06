use std::{
    cell::LazyCell,
    collections::{BTreeMap, HashMap, HashSet},
    fmt::Write,
    fs::File,
    io::BufReader,
    path::Path,
};

const ONLY_RUN: Option<&str> = None;

const BLACKLIST: LazyCell<HashSet<&'static str>> = LazyCell::new(|| {
    HashSet::<_>::from_iter([
        "oshirodoor",
        "MaxHelpingHand/OneWayInvisibleBarrierHorizontal",
        "ArphimigonHelper/WarpZone", // these read from entity.spritePath
        "ArphimigonHelper/Shark",
        "ArphimigonHelper/Collectible",
        "MaxHelpingHand/ExpandTriggerController",
        "MaxHelpingHand/Comment",
    ])
});

use anyhow::{ensure, Context, Result};
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
function drawableSpriteMt.__index.fromTexture(texture, entity)
    drawableSprite = { _type = "drawableSprite", texture = texture }
    return setmetatable(drawableSprite, drawableSpriteMt)
end
function drawableSpriteMt.__index:setJustification(justificationX, justificationY)
    if type(justificationX) == "table" then
        justificationX, justificationY = justificationX[1], justificationX[2]
    end

    self.justificationX = justificationX
    self.justificationY = justificationY

    return self
end
function drawableSpriteMt.__index:setPosition(x, y)
    if type(x) == "table" then
        x, y = x[1] or x.x, x[2] or x.y
    end

    self.x = x
    self.y = y

    return self
end

function drawableSpriteMt.__index:addPosition(x, y)
    if type(x) == "table" then
        x, y = x[1] or x.x, x[2] or x.y
    end

    self.x = self.x + x
    self.y = self.y + y

    return self
end

function drawableSpriteMt.__index:setScale(scaleX, scaleY)
    if type(scaleX) == "table" then
        scaleX, scaleY = scaleX[1], scaleX[2]
    end

    self.scaleX = scaleX
    self.scaleY = scaleY

    return self
end

function drawableSpriteMt.__index:setOffset(offsetX, offsetY)
    if type(offsetX) == "table" then
        offsetX, offsetY = offsetX[1], offsetX[2]
    end

    self.offsetX = offsetX
    self.offsetY = offsetY

    return self
end

function drawableSpriteMt.__index:setColor(color)
    -- local tableColor = utils.getColor(color)
    local tableColor = color

    if tableColor then
        self.color = tableColor
    end

    return tableColor ~= nil
end

function drawableSpriteMt.__index:setAlpha(alpha)
    local r, g, b = unpack(self.color or {})
    local newColor = {r or 1, g or 1, b or 1, alpha}

    return setColor(self, newColor)
end

setmetatable(drawableSprite, drawableSpriteMt)
package.preload["structs.drawable_sprite"] = function() return drawableSprite end

package.preload["structs.drawable_function"] = function() return nil end
package.preload["structs.drawable_line"] = function() return nil end
package.preload["structs.node"] = function() return nil end
package.preload["structs.drawable_rectangle"] = function() return nil end
package.preload["structs.drawable_nine_patch"] = function() return nil end
package.preload["utils.drawing"] = function() return nil end
package.preload["utils.matrix"] = function() return nil end
package.preload["entities"] = function() return nil end
package.preload["helpers.spikes"] = function() return nil end
package.preload["helpers.waterfalls"] = function() return nil end
package.preload["helpers.flagline"] = function() return nil end
package.preload["atlases"] = function() return nil end

requireFromPluginCalls = {}

jautils = {}
function jautils.getColor(color)
    return color
end
function jautils.getColors(colors)
    if colors == nil then
        colors = {{255,0,255}}
    end

    return colors
end

mods = {}
function mods.requireFromPlugin(plugin)
    requireFromPluginCalls[plugin] = (requireFromPluginCalls[plugin] or 0) + 1

    if plugin == "libraries.jautils" then return jautils end
    return {}
end
package.preload["mods"] = function() return mods end


utils = {}
function utils.setSimpleCoordinateSeed() end
package.preload["utils"] = function() return utils end

package.preload["consts.celeste_enums"] = function() return {} end
package.preload["consts.xna_colors"] = function() return {} end
package.preload["helpers.connected_entities"] = function() return {} end
package.preload["helpers.resort_platforms"] = function() return {} end


fakeTiles = {}
function fakeTiles.getEntitySpriteFunction(materialKey, blendKey, layer, color, x, y)
    res = { _type = "tiles", materialKey, blendKey, layer, color, x, y }
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

    let require_from_plugin_calls = lua
        .load("requireFromPluginCalls")
        .eval::<HashMap<String, u32>>()?;
    dbg!(require_from_plugin_calls);
    dbg!(stats);
    dbg!(results.len());

    let mut out = String::new();

    writeln!(
        &mut out,
        r"#![allow(clippy::approx_constant)]
use super::{{RenderMethod, RenderTexture}};
use std::collections::HashMap;
use tiny_skia::Color;

#[rustfmt::skip]
pub fn render_methods() -> HashMap<&'static str, RenderMethod> {{
    let mut textures = HashMap::new();
"
    )?;

    for (name, render) in results {
        match render {
            EntityRender::Textures(textures) => {
                write!(
                    &mut out,
                    r#"    textures.insert("{name}", RenderMethod::Textures(vec!["#,
                )?;
                for Texture {
                    texture,
                    justification,
                    rotation,
                } in textures
                {
                    write!(
                        &mut out,
                        r#"RenderTexture {{ texture: "{texture}", justification: {justification:?}, rotation: {rotation:?} }},"#
                    )?;
                }
                writeln!(&mut out, "]));")?;
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
    draw_fn: u32,
    other: u32,
    sprite_multiple: u32,
    fn_call_other: StatsFnCall,
    fn_call_sprite: StatsFnCall,
}

#[derive(Default, Debug)]
struct StatsFnCall {
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
            .field("draw_fn", &self.draw_fn)
            .field("other", &self.other)
            .field("sprite_multiple", &self.sprite_multiple)
            .field("fn_call_other", &self.fn_call_other)
            .field("fn_call_sprite", &self.fn_call_sprite)
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
    let accept = ONLY_RUN.map_or(true, |only_run| {
        file.to_ascii_lowercase()
            .contains(&only_run.to_ascii_lowercase())
    });
    if !accept {
        return Ok(());
    }

    let chunk = lua.load(chunk);

    let val = match chunk.eval::<Value>() {
        Err(error) => {
            stats.error += 1;

            if ONLY_RUN.is_some() {
                eprintln!("{:#?}", error);
            }

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

struct Texture {
    texture: String,
    justification: Option<(f32, f32)>,
    rotation: Option<f32>,
}

enum EntityRender {
    Textures(Vec<Texture>),
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

impl EntityRender {
    fn texture(texture: String, justification: Option<(f32, f32)>, rotation: Option<f32>) -> Self {
        EntityRender::Textures(vec![Texture {
            texture,
            justification,
            rotation,
        }])
    }
}
#[derive(Clone, Copy)]
struct Color([f32; 4]);
impl Color {
    fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color([
            (r as f32 * 255.0),
            (g as f32 * 255.0),
            (b as f32 * 255.0),
            (a as f32 * 255.0),
        ])
    }
}
impl<'lua> FromLua<'lua> for Color {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> mlua::prelude::LuaResult<Self> {
        match value {
            Value::String(str) => {
                let color = parse_color(str.to_str()?).unwrap();
                Ok(color)
            }
            Value::Table(vals) => {
                let mut vals = vals.sequence_values();
                let r = vals.next().unwrap().unwrap();
                let g = vals.next().unwrap().unwrap();
                let b = vals.next().unwrap().unwrap();
                let a = vals.next().unwrap_or(Ok(1.0)).unwrap();

                Ok(Color([r, g, b, a]))
            }

            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "table",
                message: None,
            }),
        }
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
    stats: &mut StatsFnCall,
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
                    Err(error) => {
                        if ONLY_RUN.is_some() {
                            eprintln!("{error}");
                        }
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

    if BLACKLIST.contains(&name.as_str()) {
        return Ok(());
    }

    let has_rectangle = table.get::<_, Option<Function>>("rectangle")?.is_some();

    let color = from_lua_or_function::<Color>(
        lua,
        table.get::<_, Value>("color")?,
        &mut stats.fn_call_other,
    )?;
    let fill = from_lua_or_function::<Color>(
        lua,
        table.get::<_, Value>("fillColor")?,
        &mut stats.fn_call_other,
    )?;
    let border = from_lua_or_function::<Color>(
        lua,
        table.get::<_, Value>("borderColor")?,
        &mut stats.fn_call_other,
    )?;

    if !has_rectangle {
        if let (Some(fill), Some(border)) = (fill, border) {
            results.insert(name.clone(), EntityRender::Rect(fill, border));
            return Ok(());
        }
    }

    let justification = from_lua_or_function::<[f32; 2]>(
        lua,
        table.get("justification")?,
        &mut stats.fn_call_other,
    )?;
    let justification = justification.map(|[x, y]| (x, y));

    let texture = from_lua_or_function::<String>(
        lua,
        table.get::<_, Value>("texture")?,
        &mut stats.fn_call_other,
    )?;

    if let Some(texture) = texture {
        let rotation =
            from_lua_or_function::<f32>(lua, table.get("rotation")?, &mut stats.fn_call_other)?;

        results.insert(
            name.clone(),
            EntityRender::texture(texture, justification, rotation),
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

    let sprite =
        from_lua_or_function::<Table>(lua, table.get("sprite")?, &mut stats.fn_call_sprite)?;

    if let Some(sprite) = sprite {
        let mut count = 0;
        sprite.for_each(|_: Value, _: Value| {
            count += 1;
            Ok(())
        })?;
        if count == 0 {
            stats.sprite_multiple += 1;
            return Ok(());
        }

        let Some(ty) = sprite.get::<_, Option<String>>("_type")? else {
            let mut textures = Vec::new();
            for val in sprite.sequence_values::<Table>() {
                let val = val?;
                let texture = drawable_sprite(&val)?;
                textures.push(texture);
            }
            results.insert(name, EntityRender::Textures(textures));
            return Ok(());
        };

        let render = match ty.as_str() {
            "tiles" => fake_tiles(&sprite)?,
            "drawableSprite" => EntityRender::Textures(vec![drawable_sprite(&sprite)?]),
            other => todo!("{}", other),
        };
        results.insert(name, render);
        return Ok(());
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

fn drawable_sprite(sprite: &Table) -> Result<Texture> {
    let _color = sprite.get::<_, Option<Color>>("color")?;
    let justification_x = sprite
        .get::<_, Option<f32>>("justificationX")?
        .unwrap_or(0.5);
    let justification_y = sprite
        .get::<_, Option<f32>>("justificationY")?
        .unwrap_or(0.5);
    let texture = sprite
        .get::<_, Option<String>>("texture")?
        .context("missing texture")?;

    Ok(Texture {
        texture,
        justification: Some((justification_x, justification_y)),
        rotation: None,
    })
}

fn fake_tiles(sprite: &Table) -> Result<EntityRender> {
    let material_key = sprite.get::<_, String>(1)?;
    let blend_key = match sprite.get::<_, Value>(2)? {
        Value::Nil => false,
        Value::String(str) if str == "blendin" => true,
        Value::Boolean(val) => val,
        other => unimplemented!("{other:?}"),
    };
    let layer = sprite.get::<_, Option<String>>(3).context("layer")?;
    let color = sprite.get::<_, Option<Color>>(4).context("color")?;
    let x = sprite.get::<_, Option<String>>(5).context("x")?;
    let y = sprite.get::<_, Option<String>>(6).context("y")?;

    Ok(EntityRender::FakeTiles {
        material_key,
        blend_key,
        layer,
        color,
        x,
        y,
    })
}

fn parse_color(color: &str) -> Result<Color> {
    match color {
        "Transparent" => return Ok(Color::from_rgba8(0, 0, 0, 0)),
        "AliceBlue" => return Ok(Color::from_rgba8(240, 248, 255, 255)),
        "AntiqueWhite" => return Ok(Color::from_rgba8(250, 235, 215, 255)),
        "Aqua" => return Ok(Color::from_rgba8(0, 255, 255, 255)),
        "Aquamarine" => return Ok(Color::from_rgba8(127, 255, 212, 255)),
        "Azure" => return Ok(Color::from_rgba8(240, 255, 255, 255)),
        "Beige" => return Ok(Color::from_rgba8(245, 245, 220, 255)),
        "Bisque" => return Ok(Color::from_rgba8(255, 228, 196, 255)),
        "Black" => return Ok(Color::from_rgba8(0, 0, 0, 255)),
        "BlanchedAlmond" => return Ok(Color::from_rgba8(255, 235, 205, 255)),
        "Blue" => return Ok(Color::from_rgba8(0, 0, 255, 255)),
        "BlueViolet" => return Ok(Color::from_rgba8(138, 43, 226, 255)),
        "Brown" => return Ok(Color::from_rgba8(165, 42, 42, 255)),
        "BurlyWood" => return Ok(Color::from_rgba8(222, 184, 135, 255)),
        "CadetBlue" => return Ok(Color::from_rgba8(95, 158, 160, 255)),
        "Chartreuse" => return Ok(Color::from_rgba8(127, 255, 0, 255)),
        "Chocolate" => return Ok(Color::from_rgba8(210, 105, 30, 255)),
        "Coral" => return Ok(Color::from_rgba8(255, 127, 80, 255)),
        "CornflowerBlue" => return Ok(Color::from_rgba8(100, 149, 237, 255)),
        "Cornsilk" => return Ok(Color::from_rgba8(255, 248, 220, 255)),
        "Crimson" => return Ok(Color::from_rgba8(220, 20, 60, 255)),
        "Cyan" => return Ok(Color::from_rgba8(0, 255, 255, 255)),
        "DarkBlue" => return Ok(Color::from_rgba8(0, 0, 139, 255)),
        "DarkCyan" => return Ok(Color::from_rgba8(0, 139, 139, 255)),
        "DarkGoldenrod" => return Ok(Color::from_rgba8(184, 134, 11, 255)),
        "DarkGray" => return Ok(Color::from_rgba8(169, 169, 169, 255)),
        "DarkGreen" => return Ok(Color::from_rgba8(0, 100, 0, 255)),
        "DarkKhaki" => return Ok(Color::from_rgba8(189, 183, 107, 255)),
        "DarkMagenta" => return Ok(Color::from_rgba8(139, 0, 139, 255)),
        "DarkOliveGreen" => return Ok(Color::from_rgba8(85, 107, 47, 255)),
        "DarkOrange" => return Ok(Color::from_rgba8(255, 140, 0, 255)),
        "DarkOrchid" => return Ok(Color::from_rgba8(153, 50, 204, 255)),
        "DarkRed" => return Ok(Color::from_rgba8(139, 0, 0, 255)),
        "DarkSalmon" => return Ok(Color::from_rgba8(233, 150, 122, 255)),
        "DarkSeaGreen" => return Ok(Color::from_rgba8(143, 188, 139, 255)),
        "DarkSlateBlue" => return Ok(Color::from_rgba8(72, 61, 139, 255)),
        "DarkSlateGray" => return Ok(Color::from_rgba8(47, 79, 79, 255)),
        "DarkTurquoise" => return Ok(Color::from_rgba8(0, 206, 209, 255)),
        "DarkViolet" => return Ok(Color::from_rgba8(148, 0, 211, 255)),
        "DeepPink" => return Ok(Color::from_rgba8(255, 20, 147, 255)),
        "DeepSkyBlue" => return Ok(Color::from_rgba8(0, 191, 255, 255)),
        "DimGray" => return Ok(Color::from_rgba8(105, 105, 105, 255)),
        "DodgerBlue" => return Ok(Color::from_rgba8(30, 144, 255, 255)),
        "Firebrick" => return Ok(Color::from_rgba8(178, 34, 34, 255)),
        "FloralWhite" => return Ok(Color::from_rgba8(255, 250, 240, 255)),
        "ForestGreen" => return Ok(Color::from_rgba8(34, 139, 34, 255)),
        "Fuchsia" => return Ok(Color::from_rgba8(255, 0, 255, 255)),
        "Gainsboro" => return Ok(Color::from_rgba8(220, 220, 220, 255)),
        "GhostWhite" => return Ok(Color::from_rgba8(248, 248, 255, 255)),
        "Gold" => return Ok(Color::from_rgba8(255, 215, 0, 255)),
        "Goldenrod" => return Ok(Color::from_rgba8(218, 165, 32, 255)),
        "Gray" => return Ok(Color::from_rgba8(128, 128, 128, 255)),
        "Green" => return Ok(Color::from_rgba8(0, 128, 0, 255)),
        "GreenYellow" => return Ok(Color::from_rgba8(173, 255, 47, 255)),
        "Honeydew" => return Ok(Color::from_rgba8(240, 255, 240, 255)),
        "HotPink" => return Ok(Color::from_rgba8(255, 105, 180, 255)),
        "IndianRed" => return Ok(Color::from_rgba8(205, 92, 92, 255)),
        "Indigo" => return Ok(Color::from_rgba8(75, 0, 130, 255)),
        "Ivory" => return Ok(Color::from_rgba8(255, 255, 240, 255)),
        "Khaki" => return Ok(Color::from_rgba8(240, 230, 140, 255)),
        "Lavender" => return Ok(Color::from_rgba8(230, 230, 250, 255)),
        "LavenderBlush" => return Ok(Color::from_rgba8(255, 240, 245, 255)),
        "LawnGreen" => return Ok(Color::from_rgba8(124, 252, 0, 255)),
        "LemonChiffon" => return Ok(Color::from_rgba8(255, 250, 205, 255)),
        "LightBlue" => return Ok(Color::from_rgba8(173, 216, 230, 255)),
        "LightCoral" => return Ok(Color::from_rgba8(240, 128, 128, 255)),
        "LightCyan" => return Ok(Color::from_rgba8(224, 255, 255, 255)),
        "LightGoldenrodYellow" => return Ok(Color::from_rgba8(250, 250, 210, 255)),
        "LightGray" => return Ok(Color::from_rgba8(211, 211, 211, 255)),
        "LightGreen" => return Ok(Color::from_rgba8(144, 238, 144, 255)),
        "LightPink" => return Ok(Color::from_rgba8(255, 182, 193, 255)),
        "LightSalmon" => return Ok(Color::from_rgba8(255, 160, 122, 255)),
        "LightSeaGreen" => return Ok(Color::from_rgba8(32, 178, 170, 255)),
        "LightSkyBlue" => return Ok(Color::from_rgba8(135, 206, 250, 255)),
        "LightSlateGray" => return Ok(Color::from_rgba8(119, 136, 153, 255)),
        "LightSteelBlue" => return Ok(Color::from_rgba8(176, 196, 222, 255)),
        "LightYellow" => return Ok(Color::from_rgba8(255, 255, 224, 255)),
        "Lime" => return Ok(Color::from_rgba8(0, 255, 0, 255)),
        "LimeGreen" => return Ok(Color::from_rgba8(50, 205, 50, 255)),
        "Linen" => return Ok(Color::from_rgba8(250, 240, 230, 255)),
        "Magenta" => return Ok(Color::from_rgba8(255, 0, 255, 255)),
        "Maroon" => return Ok(Color::from_rgba8(128, 0, 0, 255)),
        "MediumAquamarine" => return Ok(Color::from_rgba8(102, 205, 170, 255)),
        "MediumBlue" => return Ok(Color::from_rgba8(0, 0, 205, 255)),
        "MediumOrchid" => return Ok(Color::from_rgba8(186, 85, 211, 255)),
        "MediumPurple" => return Ok(Color::from_rgba8(147, 112, 219, 255)),
        "MediumSeaGreen" => return Ok(Color::from_rgba8(60, 179, 113, 255)),
        "MediumSlateBlue" => return Ok(Color::from_rgba8(123, 104, 238, 255)),
        "MediumSpringGreen" => return Ok(Color::from_rgba8(0, 250, 154, 255)),
        "MediumTurquoise" => return Ok(Color::from_rgba8(72, 209, 204, 255)),
        "MediumVioletRed" => return Ok(Color::from_rgba8(199, 21, 133, 255)),
        "MidnightBlue" => return Ok(Color::from_rgba8(25, 25, 112, 255)),
        "MintCream" => return Ok(Color::from_rgba8(245, 255, 250, 255)),
        "MistyRose" => return Ok(Color::from_rgba8(255, 228, 225, 255)),
        "Moccasin" => return Ok(Color::from_rgba8(255, 228, 181, 255)),
        "NavajoWhite" => return Ok(Color::from_rgba8(255, 222, 173, 255)),
        "Navy" => return Ok(Color::from_rgba8(0, 0, 128, 255)),
        "OldLace" => return Ok(Color::from_rgba8(253, 245, 230, 255)),
        "Olive" => return Ok(Color::from_rgba8(128, 128, 0, 255)),
        "OliveDrab" => return Ok(Color::from_rgba8(107, 142, 35, 255)),
        "Orange" => return Ok(Color::from_rgba8(255, 165, 0, 255)),
        "OrangeRed" => return Ok(Color::from_rgba8(255, 69, 0, 255)),
        "Orchid" => return Ok(Color::from_rgba8(218, 112, 214, 255)),
        "PaleGoldenrod" => return Ok(Color::from_rgba8(238, 232, 170, 255)),
        "PaleGreen" => return Ok(Color::from_rgba8(152, 251, 152, 255)),
        "PaleTurquoise" => return Ok(Color::from_rgba8(175, 238, 238, 255)),
        "PaleVioletRed" => return Ok(Color::from_rgba8(219, 112, 147, 255)),
        "PapayaWhip" => return Ok(Color::from_rgba8(255, 239, 213, 255)),
        "PeachPuff" => return Ok(Color::from_rgba8(255, 218, 185, 255)),
        "Peru" => return Ok(Color::from_rgba8(205, 133, 63, 255)),
        "Pink" => return Ok(Color::from_rgba8(255, 192, 203, 255)),
        "Plum" => return Ok(Color::from_rgba8(221, 160, 221, 255)),
        "PowderBlue" => return Ok(Color::from_rgba8(176, 224, 230, 255)),
        "Purple" => return Ok(Color::from_rgba8(128, 0, 128, 255)),
        "Red" => return Ok(Color::from_rgba8(255, 0, 0, 255)),
        "RosyBrown" => return Ok(Color::from_rgba8(188, 143, 143, 255)),
        "RoyalBlue" => return Ok(Color::from_rgba8(65, 105, 225, 255)),
        "SaddleBrown" => return Ok(Color::from_rgba8(139, 69, 19, 255)),
        "Salmon" => return Ok(Color::from_rgba8(250, 128, 114, 255)),
        "SandyBrown" => return Ok(Color::from_rgba8(244, 164, 96, 255)),
        "SeaGreen" => return Ok(Color::from_rgba8(46, 139, 87, 255)),
        "SeaShell" => return Ok(Color::from_rgba8(255, 245, 238, 255)),
        "Sienna" => return Ok(Color::from_rgba8(160, 82, 45, 255)),
        "Silver" => return Ok(Color::from_rgba8(192, 192, 192, 255)),
        "SkyBlue" => return Ok(Color::from_rgba8(135, 206, 235, 255)),
        "SlateBlue" => return Ok(Color::from_rgba8(106, 90, 205, 255)),
        "SlateGray" => return Ok(Color::from_rgba8(112, 128, 144, 255)),
        "Snow" => return Ok(Color::from_rgba8(255, 250, 250, 255)),
        "SpringGreen" => return Ok(Color::from_rgba8(0, 255, 127, 255)),
        "SteelBlue" => return Ok(Color::from_rgba8(70, 130, 180, 255)),
        "Tan" => return Ok(Color::from_rgba8(210, 180, 140, 255)),
        "Teal" => return Ok(Color::from_rgba8(0, 128, 128, 255)),
        "Thistle" => return Ok(Color::from_rgba8(216, 191, 216, 255)),
        "Tomato" => return Ok(Color::from_rgba8(255, 99, 71, 255)),
        "Turquoise" => return Ok(Color::from_rgba8(64, 224, 208, 255)),
        "Violet" => return Ok(Color::from_rgba8(238, 130, 238, 255)),
        "Wheat" => return Ok(Color::from_rgba8(245, 222, 179, 255)),
        "White" => return Ok(Color::from_rgba8(255, 255, 255, 255)),
        "WhiteSmoke" => return Ok(Color::from_rgba8(245, 245, 245, 255)),
        "Yellow" => return Ok(Color::from_rgba8(255, 255, 0, 255)),
        "YellowGreen" => return Ok(Color::from_rgba8(154, 205, 50, 255)),
        _ => {}
    };

    ensure!(color.len() == 6, "unknown color: {color}");

    assert_eq!(color.len(), 6);

    let r = u8::from_str_radix(&color[..=1], 16)?;
    let g = u8::from_str_radix(&color[2..=3], 16)?;
    let b = u8::from_str_radix(&color[4..=5], 16)?;

    Ok(Color::from_rgba8(r, g, b, 255))
}
