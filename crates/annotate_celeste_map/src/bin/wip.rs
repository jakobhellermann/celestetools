use std::{collections::HashMap, fs::File, io::BufReader, time::Instant};

use annotate_celeste_map::cct_physics_inspector::PhysicsInspector;
use anyhow::Result;
use celesteloader::{archive::ModArchive, map::Bounds, CelesteInstallation};
use celesterender::{
    asset::{AssetDb, ModLookup},
    CelesteRenderData,
};
use tiny_skia::{
    Color, GradientStop, LinearGradient, Paint, PathBuilder, Pixmap, Point, Rect, Stroke, Transform,
};

fn main() -> Result<()> {
    let celeste = CelesteInstallation::detect()?;
    let physics_inspector = PhysicsInspector::new(&celeste);

    let term = std::env::args().nth(1).unwrap_or("".into()).to_lowercase();

    let sids: HashMap<String, Vec<u32>> = physics_inspector
        .recent_recordings()?
        .into_iter()
        .filter(|(_, layout)| layout.chapter_name.to_lowercase().contains(&term))
        .fold(HashMap::new(), |mut acc, (i, layout)| {
            acc.entry(layout.sid.unwrap()).or_default().push(i);
            acc
        });

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
        let (bounds, mut image) =
            celesterender::render_map_sid(&celeste, &mut render_data, &mut asset_db, &sid)?;

        for recording in recordings {
            annotate_cct_recording_skia(&mut image, &physics_inspector, recording, bounds)?;
        }

        image.save_png(format!("{}.png", sid.replace(['/'], "_")))?;

        println!("Rendered map {sid} in {:.2}ms", a.elapsed().as_millis());
    }

    Ok(())
}

fn annotate_cct_recording_skia(
    image: &mut Pixmap,
    physics_inspector: &PhysicsInspector,
    i: u32,
    bounds: Bounds,
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

    image.stroke_path(
        &path,
        &Paint {
            shader: gradient,
            blend_mode: tiny_skia::BlendMode::SourceOver,
            anti_alias: true,
            ..Default::default()
        },
        &Stroke {
            width: 10.0,
            line_cap: tiny_skia::LineCap::Butt,
            line_join: tiny_skia::LineJoin::Round,
            ..Default::default()
        },
        map2img,
        None,
    );

    Ok(())
}
