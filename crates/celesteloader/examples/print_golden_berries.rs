use anyhow::Result;
use celesteloader::{map::utils::parse_map_name, CelesteInstallation};
use std::collections::BTreeMap;

#[derive(Default)]
struct Modes {
    a: Vec<String>,
    b: Vec<String>,
    c: Vec<String>,
}

fn main() -> Result<()> {
    let celeste = CelesteInstallation::detect()?;

    let mut goldens: BTreeMap<_, Modes> = BTreeMap::new();
    for map in celeste.vanilla_maps()? {
        let room_goldens = map
            .rooms
            .iter()
            .flat_map(|room| {
                let room_name = room.name.strip_prefix("lvl_").unwrap();
                room.entities
                    .iter()
                    .filter(|entity| {
                        entity.name == "goldenBerry" || entity.name == "memorialTextController"
                    })
                    .map(move |golden| format!("{room_name}:{}", golden.id.unwrap()))
            })
            .collect::<Vec<_>>();

        let (order, side, name) = parse_map_name(&map.package);
        let name = match order {
            Some(order) => format!("{order}-{name}"),
            None => name.into(),
        };

        let entry = goldens.entry(name).or_default();

        match side {
            None => entry.a = room_goldens,
            Some('H') => entry.b = room_goldens,
            Some('X') => entry.c = room_goldens,
            _ => unreachable!("{:?} in {}", side, map.package),
        }
    }

    for (name, goldens) in goldens {
        println!(
            r#"{{ "Celeste/{name}", [{:?}, {:?}, {:?}] }},"#,
            goldens.a, goldens.b, goldens.c
        );
    }
    Ok(())
}
