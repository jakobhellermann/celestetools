use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::Path,
    time::{Duration, Instant},
};

use annotate_celeste_map::cct_physics_inspector::PhysicsInspector;
use anyhow::{bail, ensure, Context, Result};
use celestedebugrc::DebugRC;
use celesteloader::{
    archive::ModArchive, map::Bounds, utils::list_dir_extension, CelesteInstallation,
};
use celesterender::{
    asset::{AssetDb, ModLookup},
    CelesteRenderData,
};
use clap::{Parser, ValueEnum};
use tiny_skia::{
    Color, GradientStop, LinearGradient, Paint, PathBuilder, Pixmap, Point, Rect, Stroke, Transform,
};

#[derive(Debug, Clone, ValueEnum)]
enum ColorMode {
    Gradient,
    Random,
}

#[derive(Debug, Parser)]
struct App {
    #[clap(long = "filter", num_args = 0.., help =
        r#"Filter which recordings should be included, e.g. 'city' or 'beginner lobby'"#,
    )]
    filter: Option<Vec<String>>,

    #[clap(long = "record", num_args = 0.., help =
        r#"Record TASes first (--record alone will record TASes in current directory)"#,
    )]
    record: Option<Vec<String>>,

    #[clap(long = "open", help = "Open file after annotating")]
    open: bool,

    #[clap(flatten, next_help_heading = "Ui")]
    ui: UiArgs,
}
#[derive(Debug, clap::Args)]
struct UiArgs {
    #[clap(long = "width", help = "Width of the line")]
    width: Option<f32>,

    #[clap(long = "color", default_value = "gradient")]
    color: ColorMode,
}

fn main() {
    let mut args = App::parse();
    if let Some(filters) = &mut args.filter {
        *filters = filters
            .iter_mut()
            .flat_map(|args| args.split(','))
            .map(ToOwned::to_owned)
            .collect();
    }

    if let Err(e) = run(args) {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
}

fn record_folder(folder: impl AsRef<Path>) -> Result<()> {
    let folder = folder.as_ref();
    let debugrc = DebugRC::new();

    let mut tas_files = Vec::new();

    if folder.is_file() {
        tas_files.push(folder.to_path_buf());
    } else {
        let mut empty = true;
        list_dir_extension::<_, anyhow::Error>(folder.as_ref(), "tas", |tas| {
            empty = false;
            tas_files.push(tas.to_path_buf());

            Ok(())
        })?;
        ensure!(!empty, "No TAS files found in folder {}", folder.display());
    }

    let enforce_legal = tas_files.iter().fold(false, |acc, file| {
        let content = std::fs::read_to_string(&file).unwrap_or_default();
        acc || content.contains("EnforceLegal") || content.contains("EnforceMaingame")
    });

    if enforce_legal {
        eprintln!("File contains EnforceLegal, falling back to running TASes one by one");
    }

    let speedup = 500;
    let tmp_files = if enforce_legal {
        tas_files
            .iter()
            .map(|file| {
                let name = file.file_name().unwrap().to_str().unwrap();
                let file = file.to_str().unwrap();
                (format!("Read,{file}\n***{speedup}"), Some(name))
            })
            .collect()
    } else {
        let mut temp_content = tas_files
            .iter()
            .map(|path| format!("Read,{}\n", path.to_str().unwrap()))
            .collect::<String>();
        temp_content.push_str("\n***{speedup}");
        vec![(temp_content, None)]
    };

    let start = Instant::now();
    for (content, origin) in tmp_files {
        let path = std::env::temp_dir().join("tmp.tas");

        std::fs::write(&path, content)?;
        debugrc.play_tas_sync(&path, |info| {
            let current = find_info(info, "CurrentFrame: ");
            let total = find_info(info, "TotalFrames: ");
            if let Some(origin) = origin {
                eprintln!("{origin}: {}/{}", current, total);
            } else {
                eprintln!("{}/{}", current, total);
            }
        })?;
        debugrc.respawn()?;
        std::fs::remove_file(&path)?;
    }
    dbg!(start.elapsed());

    Ok(())
}

fn find_info<'a>(str: &'a str, prop: &str) -> &'a str {
    let Some(i) = str.find(prop) else { return "" };
    let str = &str[i + prop.len()..];

    let idx_newline = str.find("<br").unwrap_or(str.len());
    &str[..idx_newline]
}

fn run(args: App) -> Result<()> {
    let celeste = CelesteInstallation::detect()?;
    let physics_inspector = PhysicsInspector::new(&celeste);

    if let Some(args) = args.record {
        if args.is_empty() {
            let path = std::env::current_dir()?;
            record_folder(&path)?;
        }
        for arg in args {
            record_folder(&arg)?;
        }
        std::thread::sleep(Duration::from_millis(500));
    }

    let mut sids: HashMap<String, Vec<u32>> = HashMap::new();
    for (i, layout) in physics_inspector.recent_recordings()? {
        if !matches_filter(i, &layout.chapter_name, args.filter.as_deref()) {
            continue;
        }

        let Some(sid) = layout.sid else {
            eprintln!(
                "Recording {i} in {} was recorded using a too old version of Physics Inspector, skipping",
                layout.chapter_name
            );
            continue;
        };

        sids.entry(sid).or_default().push(i);
    }
    if sids.is_empty() {
        bail!("no physics recordings found");
    }

    let mods = celeste
        .list_mod_zips()?
        .into_iter()
        .map(File::open)
        .collect::<Result<Vec<_>, std::io::Error>>()?;
    let mut mods = mods
        .into_iter()
        .map(|data| ModArchive::new(BufReader::new(data)))
        .collect::<Result<Vec<_>, _>>()?;
    let mut asset_db = AssetDb::new(ModLookup::new(mods.as_mut_slice(), &celeste));
    let mut render_data = CelesteRenderData::base(&celeste)?;

    for (sid, recordings) in sids {
        let a = Instant::now();
        let (mut result, map) =
            celesterender::render_map_sid(&celeste, &mut render_data, &mut asset_db, &sid)
                .with_context(|| format!("error rendering {sid}"))?;

        if result.unknown_entities.len() > 0 {
            let mut unknown = result.unknown_entities.into_iter().collect::<Vec<_>>();
            unknown.sort_by_key(|&(_, n)| std::cmp::Reverse(n));

            eprintln!(
                "found {} unknown entities: ({} ...)\n",
                unknown.len(),
                unknown
                    .iter()
                    .take(3)
                    .map(|(name, num)| format!("{num} {name} "))
                    .collect::<String>()
            );
        }

        let size_filled = map.rooms.iter().map(|room| room.bounds.area()).sum::<f32>();
        let size = result.bounds.area();
        let density = size_filled / size;

        for recording in recordings {
            annotate_cct_recording_skia(
                &mut result.image,
                &physics_inspector,
                recording,
                result.bounds,
                density,
                &args.ui,
            )?;
        }

        let out_path = format!("{}.png", sid.replace(['/'], "_"));
        result.image.save_png(&out_path)?;
        println!("Rendered map {sid} in {:.2}ms", a.elapsed().as_millis());

        if args.open {
            opener::open(&out_path)?;
        }
    }

    Ok(())
}

fn annotate_cct_recording_skia(
    image: &mut Pixmap,
    physics_inspector: &PhysicsInspector,
    i: u32,
    bounds: Bounds,
    density: f32,
    args: &UiArgs,
) -> Result<()> {
    let position_log = physics_inspector.position_log(i)?;

    let mut path = Vec::new();
    for log in position_log {
        let (x, y, flags) = log?;
        let state = flags.split(' ').next().unwrap().to_owned();

        let new_entry = (x, y, state);
        let same_as_last = path.last() == Some(&new_entry);
        if !same_as_last {
            path.push(new_entry);
        }
    }

    if path.len() <= 1 {
        return Ok(());
    }

    let mut pb = PathBuilder::new();

    let mut path = path.into_iter();
    let (start_x, start_y, _) = path.next().unwrap();
    pb.move_to(start_x, start_y);

    for (x, y, _) in path {
        pb.line_to(x, y);
    }

    let path = pb.finish().unwrap();

    let gradient = LinearGradient::new(
        Point::from_xy(0.0, 0.0),
        Point::from_xy(bounds.size.0 as f32, bounds.size.1 as f32),
        vec![
            GradientStop::new(0.0, Color::from_rgba8(255, 0, 0, 255)),
            GradientStop::new(0.5, Color::from_rgba8(128, 0, 128, 255)),
            GradientStop::new(1.0, Color::from_rgba8(15, 30, 150, 255)),
        ],
        tiny_skia::SpreadMode::Reflect,
        Transform::identity(),
    )
    .unwrap();

    if false {
        image.fill_rect(
            Rect::from_ltrb(0.0, 0.0, bounds.size.0 as f32, bounds.size.1 as f32).unwrap(),
            &Paint {
                shader: gradient.clone(),
                ..Default::default()
            },
            Transform::identity(),
            None,
        );
    }

    let map2img = Transform::from_translate(-bounds.position.x as f32, -bounds.position.y as f32);

    let width = args
        .width
        .unwrap_or_else(|| if density > 0.5 { 8.0 } else { 3.0 });
    image.stroke_path(
        &path,
        &Paint {
            shader: gradient,
            blend_mode: tiny_skia::BlendMode::SourceOver,
            anti_alias: true,
            ..Default::default()
        },
        &Stroke {
            width,
            line_cap: tiny_skia::LineCap::Butt,
            line_join: tiny_skia::LineJoin::Round,
            ..Default::default()
        },
        map2img,
        None,
    );

    Ok(())
}

fn matches_filter(i: u32, name: &str, filter: Option<&[String]>) -> bool {
    let Some(filter) = filter else { return true };
    let name = name.to_ascii_lowercase();

    filter
        .iter()
        .any(|filter| name.contains(&filter.to_ascii_lowercase()) || i.to_string() == *filter)
}
