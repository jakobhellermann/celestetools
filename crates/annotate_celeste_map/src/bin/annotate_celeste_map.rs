use std::{collections::HashSet, ffi::OsStr, path::PathBuf};

use annotate_celeste_map::Annotate;
use anyhow::{bail, Context, Result};
use celesteloader::{
    cct_physics_inspector::{MapBounds, PhysicsInspector},
    CelesteInstallation,
};
use clap::{builder::TypedValueParser, Parser};
use image::GenericImageView;
use paris::{error, info, success, warn};

#[derive(Debug, clap::Parser)]
struct App {
    #[clap(help = "path to the .png map export from Lönn")]
    map: PathBuf,

    #[clap(short = 'o', help = "Write annotated png to <OUTPUT>")]
    output: PathBuf,

    #[clap(long = "top-left", allow_hyphen_values = true, required_unless_present = "recent_cct_recordings", help = "tile position x,y of the topleft corner of the map", value_parser=U32CommaU32ValueParser)]
    top_left: Option<(i32, i32)>,

    #[clap(long = "open", help = "Open file after annotating")]
    open: bool,

    #[clap(flatten, next_help_heading = "Annotations")]
    annotations: AnnotationArgs,
}

#[derive(Debug, clap::Args)]
#[group(required = true)]
struct AnnotationArgs {
    #[clap(long = "cct-recordings", num_args = 0.., value_name = "FILTER", help =
        r#"Annotate with the movement of recent physics inspector recordings.
<FILTER> can be empty to select all recordings,
or 'city' to only match recordings in the given chapter,
or '1,2,5' to include specific recent recordings."#,
    )]
    recent_cct_recordings: Option<Vec<String>>,

    #[clap(
        long = "lobby-entrances",
        help = "path to .csv file with columns 'index,name,xpos,ypos'"
    )]
    lobby_entrances: Option<PathBuf>,
}

fn main() {
    let mut args = App::parse();
    if let Some(filters) = &mut args.annotations.recent_cct_recordings {
        *filters = filters
            .iter_mut()
            .flat_map(|args| args.split(','))
            .map(ToOwned::to_owned)
            .collect();
    }

    if let Err(e) = annotate(args) {
        error!("{:?}", e);
        std::process::exit(1);
    }
}

fn annotate(args: App) -> Result<()> {
    let font_data: &[u8] = include_bytes!("../../DejaVuSans.ttf");
    let font = rusttype::Font::try_from_bytes(font_data).unwrap();

    let installation = CelesteInstallation::detect_multiple()?;
    let installation = match &installation.as_slice() {
        [single] => single,
        [first, ..] => {
            info!(
                "detected {} celeste installations, using {}",
                installation.len(),
                first.path.display()
            );
            first
        }
        [] => bail!("could not find celeste installation"),
    };

    let map = image::io::Reader::open(&args.map)
        .with_context(|| format!("failed to read {}", args.map.display()))?
        .decode()?;
    let image_dimensions = map.dimensions();

    let physics_inspector = PhysicsInspector::new(installation);
    let mut recent_recordings = physics_inspector.recent_recordings()?;
    recent_recordings.sort_by_key(|(i, _)| *i);

    let infer_map_bounds = args.top_left.is_none();
    let mut map_bounds = args
        .top_left
        .map(|(x, y)| MapBounds::from_pos_width((x * 8, y * 8), image_dimensions));

    let mut matching_cct_logs = Vec::new();
    if let Some(cct_recording_filter) = args.annotations.recent_cct_recordings {
        let mut cct_chapters = HashSet::new();

        let mut skipped_dim = HashSet::new();
        let mut n_skipped_dim = 0;

        let mut skipped_filter = HashSet::new();
        let mut n_skipped_filter = 0;

        let mut matched = HashSet::new();
        let mut matched_i = HashSet::new();
        let mut n_matched = 0;

        for (i, layout) in recent_recordings {
            if !cct_recording_filter.is_empty() {
                let matches_filter = matches_filter(i, &layout.chapter_name, &cct_recording_filter);

                if !matches_filter {
                    skipped_filter.insert(layout.chapter_name.clone());
                    n_skipped_filter += 1;
                    continue;
                }
            }

            if infer_map_bounds {
                let bounds = layout.bounds();
                if bounds.dimensions() != image_dimensions {
                    skipped_dim.insert(layout.chapter_name.clone());
                    n_skipped_dim += 1;
                    continue;
                }

                match &map_bounds {
                    Some(map_bounds) => {
                        if *map_bounds != bounds {
                            bail!(
                        "CCT recording {i} ({}) has different map bounds: {bounds} != {map_bounds}",
                        layout.chapter_name,
                    );
                        }
                    }
                    _ => map_bounds = Some(bounds),
                };
            }

            /*info!(
                "CCT recording {i} in <bold>{}</> matches filter",
                layout.chapter_name
            );*/

            matched.insert(layout.chapter_name.clone());
            matched_i.insert(i);
            n_matched += 1;

            matching_cct_logs.push(i);
            cct_chapters.insert(layout.chapter_name);
        }

        if n_matched > 0 {
            info!(
                "{n_matched} CCT recording{s} match{ed} filter (<b>{}</b>)",
                matched.into_iter().collect::<Vec<_>>().join(", "),
                s = if n_matched == 1 { "s" } else { "" },
                ed = if n_matched == 1 { "s" } else { "" },
            );
        }

        if n_skipped_dim > 0 {
            warn!(
                "{n_skipped_dim} CCT recording{} skipped (<b>{}</b>) since {} match image dimensions ({}x{} tiles)",
                if n_skipped_dim == 1 { "s" } else { "" },
                skipped_dim.into_iter().collect::<Vec<_>>().join(", "),
                if n_skipped_dim == 1 {
                    "it doesn't"
                } else {
                    "they don't"
                },
                image_dimensions.0 / 8,
                image_dimensions.1 / 8
            );
        }
        #[allow(clippy::overly_complex_bool_expr)]
        if n_skipped_filter > 0 && false {
            warn!(
                "{n_skipped_filter} CCT recordings skippe{s} (<b>{}</b>) since they didn't match the filter",
                skipped_filter.into_iter().collect::<Vec<_>>().join(", "),
                s = if n_skipped_dim == 1 { "s" } else { "" },
            );
        }

        if cct_chapters.len() > 1 {
            warn!(
                "<bold>--cct-recordings</> matched recordings from multiple maps: <bold>{maps}</>. If this isn't intended, specify a filter like <bold>--cct-recordings '{instead}'</>",
                instead = cct_chapters.iter().next().unwrap().to_lowercase(),
                maps = cct_chapters.into_iter().collect::<Vec<_>>().join(", "),
            )
        }
    }

    let map_bounds = map_bounds.context(
        r#"If the CCT recording does not visit the outermost 4 rooms, you need to specify the map offset manually using e.g. <red><bold>--top-left 0,-401</>
To figure out this offset, open the debug map, find the <i>leftmost</i> room and copy the x value of the room position:
<bold>320x180  <red>0<//>,0  0,0</>
then find the <i>topmost</i> room and copy the y value of the room position:
<bold>320x180  480,<red>-401<//>  3840,-3208</>
"#
    )?;
    let mut annotate = Annotate::new(map, map_bounds);

    for i in matching_cct_logs {
        annotate.annotate_cct_recording(&physics_inspector, i)?;
    }

    if let Some(entrances) = &args.annotations.lobby_entrances {
        annotate.annotate_entries(entrances, &font)?;
    }

    annotate.save(&args.output)?;

    success!("Annotated png saved to {}", args.output.display());

    if args.open {
        opener::open(&args.output)?;
    }

    /*Annotate::map(
        "mausoleum.png",
        Anchor::BottomLeft {
            room_pos: (-128 * 8, 36 * 8),
            room_height: 23 * 8,
        },
    )?
    .save("out.png")?;*/

    /*Annotate::map(
        "testing/annotate/map.png",
        Anchor::TopLeft {
            room_pos: (87 * 8, -84 * 8),
        },
    )?
    .annotate_entries("testing/annotate/maps.csv")?
    .annotate_recent_cct_recordings(|chapter_name| chapter_name == "Flowing Gallery")?
    .save("out.png")?;

    Annotate::map(
        "testing/annotate/1a.png",
        Anchor::BottomLeft {
            room_pos: (0, 0),
            room_height: 23 * 8,
        },
    )?
    .annotate_recent_cct_recordings(|chapter_name| chapter_name == "Forsaken City")?
    .save("out.png")?;*/

    /*Annotate::map(
        "mausoleum.png",
        Anchor::BottomLeft {
            room_pos: (-128 * 8, 36 * 8),
            room_height: 23 * 8,
        },
    )?
    .annotate_recent_cct_recordings(|chapter_name| chapter_name == "The Mausoleum")?
    .save("out.png")?;*/

    Ok(())
}

fn matches_filter(i: u32, name: &str, filter: &[String]) -> bool {
    let name = name.to_ascii_lowercase();

    filter
        .iter()
        .any(|filter| name.contains(&filter.to_ascii_lowercase()) || i.to_string() == *filter)
}

#[derive(Clone)]
struct U32CommaU32ValueParser;

impl TypedValueParser for U32CommaU32ValueParser {
    type Value = (i32, i32);

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let Some(value) = value.to_str() else {
            return Err(clap::Error::new(clap::error::ErrorKind::ValueValidation));
        };

        let Some((x, y)) = value.split_once(',') else {
            return Err(clap::Error::raw(
                clap::error::ErrorKind::ValueValidation,
                format_args!("'{value}' is not a valid x,y offset"),
            )
            .with_cmd(cmd));
        };

        let inner = clap::value_parser!(i32);

        let x = inner.parse_ref(cmd, arg, OsStr::new(x))?;
        let y = inner.parse_ref(cmd, arg, OsStr::new(y))?;

        Ok((x, y))
    }
}
