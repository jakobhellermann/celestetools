use std::{
    collections::HashMap,
    path::Path,
    time::{Duration, Instant},
};

use annotate_celeste_map::LineSettings;
use anyhow::{bail, ensure, Context, Result};
use celestedebugrc::DebugRC;
use celesteloader::{
    cct_physics_inspector::PhysicsInspector, utils::list_dir_extension, CelesteInstallation,
};
use celesterender::{
    asset::{AssetDb, ModLookup},
    CelesteRenderData, RenderMapSettings,
};
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ColorMode {
    Gradient,
    State,
}
impl From<ColorMode> for annotate_celeste_map::ColorMode {
    fn from(value: ColorMode) -> Self {
        match value {
            ColorMode::Gradient => annotate_celeste_map::ColorMode::Gradient,
            ColorMode::State => annotate_celeste_map::ColorMode::State,
        }
    }
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
        list_dir_extension::<_, anyhow::Error>(folder, "tas", |tas| {
            empty = false;
            tas_files.push(tas.to_path_buf());

            Ok(())
        })?;
        ensure!(!empty, "No TAS files found in folder {}", folder.display());
    }

    let run_as_merged = false;
    debugrc.run_tases_fastforward(&tas_files, 500.0, run_as_merged, |status| {
        if let Some(origin) = status.origin {
            eprintln!("{origin}: {}/{}", status.current_frame, status.total_frames);
        } else {
            eprintln!("{}/{}", status.current_frame, status.total_frames);
        }
    })?;

    Ok(())
}

fn run(args: App) -> Result<()> {
    let celeste = CelesteInstallation::detect()?;
    let physics_inspector = PhysicsInspector::new(&celeste);

    if let Some(args) = args.record {
        if args.is_empty() {
            let path = std::env::current_dir()?;
            record_folder(path)?;
        }
        for arg in args {
            record_folder(&arg)?;
        }
        std::thread::sleep(Duration::from_millis(500));
    }

    let mut map_bins: HashMap<String, Vec<u32>> = HashMap::new();
    for (i, layout) in physics_inspector.recent_recordings()? {
        if !matches_filter(i, &layout.chapter_name, args.filter.as_deref()) {
            continue;
        }

        let Some(map_bin) = layout.map_bin else {
            eprintln!(
                "Recording {i} in {} was recorded using a too old version of Physics Inspector, skipping",
                layout.chapter_name
            );
            continue;
        };

        map_bins.entry(map_bin).or_default().push(i);
    }
    if map_bins.is_empty() {
        bail!("no physics recordings found");
    }

    let mut asset_db = AssetDb::new(ModLookup::all_mods(&celeste)?);
    let mut render_data = CelesteRenderData::base(&celeste)?;

    for (map_bin, recordings) in map_bins {
        let a = Instant::now();
        let (mut result, map) = celesterender::render_map_bin(
            &celeste,
            &mut render_data,
            &mut asset_db,
            &map_bin,
            RenderMapSettings::default(),
        )
        .with_context(|| format!("error rendering {map_bin}"))?;

        if !result.unknown_entities.is_empty() {
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
            let width = args
                .ui
                .width
                .unwrap_or(if density > 0.5 { 8.0 } else { 3.0 });

            annotate_celeste_map::annotate_cct_recording_skia(
                &mut result.image,
                &physics_inspector,
                [recording].into_iter(),
                result.bounds,
                LineSettings {
                    width,
                    color_mode: args.ui.color.into(),
                    ..Default::default()
                },
            )?;
        }

        let out_path = format!("{}.png", map_bin.replace(['/'], "_"));
        result.image.save_png(&out_path)?;
        println!("Rendered map {map_bin} in {:.2}ms", a.elapsed().as_millis());

        if args.open {
            opener::open(&out_path)?;
        }
    }

    Ok(())
}

fn matches_filter(i: u32, name: &str, filter: Option<&[String]>) -> bool {
    let Some(filter) = filter else { return true };
    let name = name.to_ascii_lowercase();

    filter
        .iter()
        .any(|filter| name.contains(&filter.to_ascii_lowercase()) || i.to_string() == *filter)
}
