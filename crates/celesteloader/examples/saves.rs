use anyhow::Result;
use celesteloader::CelesteInstallation;

fn main() -> Result<()> {
    let celeste = CelesteInstallation::detect()?;

    let settings = celeste.mod_settings("TASRecorder")?;
    let out_dir = settings["OutputDirectory"].as_str().unwrap();
    dbg!(&out_dir);

    for save in celeste.saves()? {
        let name = save.xml(|document| {
            document
                .root_element()
                .children()
                .find_map(|child| {
                    child
                        .has_tag_name("Name")
                        .then(|| child.text().unwrap().to_owned())
                })
                .unwrap()
        })?;
        println!("{} - {}", save.index(), name);
    }

    Ok(())
}
