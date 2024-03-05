use anyhow::{Context, Result};
use celesteloader::dialog::Dialog;

fn main() -> Result<()> {
    let file = std::env::args().nth(1).context("no file passed")?;

    let contents = std::fs::read_to_string(file)?;
    let dialog = Dialog::from_txt(&contents);

    println!("{:#?}", dialog.get("VanillaContest2023_0_Lobbies_Lobby"));

    Ok(())
}
