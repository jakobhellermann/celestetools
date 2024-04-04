use anyhow::Result;
use celesteloader::{
    cct_physics_inspector::compare_timesave::compare_timesave, CelesteInstallation,
};

fn main() -> Result<()> {
    let celeste = CelesteInstallation::detect()?;
    let pi = celeste.physics_inspector();

    let recordings = pi.recent_recordings_by_map_bin()?;

    for (map_bin, recordings) in recordings {
        let (map, archive) = celeste.find_map_by_map_bin(&map_bin)?;

        let map_name = archive
            .map(|mut archive| -> Result<_> {
                let dialog = archive.get_dialog("English")?;
                let map_name = dialog.get(&map_bin).unwrap_or(&map_bin);
                Ok(map_name.to_owned())
            })
            .transpose()?
            .unwrap_or_else(|| map_bin.clone());

        if recordings.len() != 2 {
            continue;
        }

        let a = compare_timesave(&pi, &map, &map_name, (recordings[0], recordings[1]))?;
        print!("{a}");
    }

    Ok(())
}
