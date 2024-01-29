use std::{
    collections::{BTreeMap, HashSet},
    ffi::OsStr,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::{Context, Result};
use cmaploader::archive::ModArchive;

fn main() -> Result<()> {
    let respect_blacklist = false;

    let mut files = Vec::new();
    for file in std::env::args().skip(1) {
        let file = PathBuf::from(file);
        if file.is_dir() {
            let blacklist = File::open(file.join("blacklist.txt"))
                .and_then(|file| {
                    BufReader::new(file)
                        .lines()
                        .collect::<Result<HashSet<String>, _>>()
                })
                .unwrap_or_default();

            for child in file.read_dir()? {
                let child = child?.path();

                if respect_blacklist
                    && child
                        .file_name()
                        .and_then(OsStr::to_str)
                        .map_or(false, |path| blacklist.contains(path))
                {
                    continue;
                }

                files.push(child);
            }
        } else {
            files.push(file);
        }
    }

    let mut level_set_names = Vec::new();
    for path in files {
        if !path.extension().map_or(false, |e| e == "zip") {
            continue;
        }

        let reader = BufReader::new(File::open(&path)?);
        let mut archive = ModArchive::new(reader)
            .with_context(|| format!("failed to read zip {}", path.display()))?;

        let everest_yaml = archive
            .everest_yaml()
            .with_context(|| format!("no everest.yaml in {}", path.display()))?;
        let everest_name = everest_yaml.lines().find_map(|name| {
            name.split_once("Name:")
                .map(|(_, val)| val.trim().to_owned())
        });

        let dialog = archive.get_dialog("English").ok();

        let is_collab = archive.is_collab();
        let zip = path.file_name().and_then(OsStr::to_str).map(String::from);

        let mut levelsets = BTreeMap::new();
        let mut areas = BTreeMap::<String, Vec<_>>::new();

        for path in archive
            .list_files()
            .filter_map(|path| path.strip_prefix("Maps/"))
        {
            let Some((levelset, area)) = path.rsplit_once('/') else {
                continue;
            };
            if levelset.matches('/').count() > 2 {
                continue;
            }

            let levelset_name = dialog.as_ref().and_then(|dialog| dialog.get(&levelset));
            levelsets.entry(levelset).or_insert(levelset_name);

            if let Some(area) = area.strip_suffix(".bin") {
                if let Some(area_name) = dialog
                    .as_ref()
                    .and_then(|dialog| dialog.get(&format!("{levelset}/{area}")))
                {
                    areas
                        .entry(levelset.to_owned())
                        .or_default()
                        .push(area_name);
                }
            }
        }

        level_set_names.extend(levelsets.iter().map(|(&set, &name)| {
            let first_levelset_name = levelsets.values().find_map(|val| *val);

            let preferred_name = match is_collab {
                false => name.or(first_levelset_name),
                true => first_levelset_name.or(name),
            };

            let name = preferred_name.or(everest_name.as_deref()).unwrap_or(&set);
            (set.to_string(), name.to_string(), zip.clone())
        }));
    }

    level_set_names.sort_by_key(|(set, ..)| set.to_ascii_lowercase());

    for (set, name, _path) in level_set_names {
        println!(
            r#"{{ "{}", "{}" }}, "#,
            set.replace('"', "\\\""),
            name.replace('"', "\\\"")
        );
    }

    Ok(())
}
