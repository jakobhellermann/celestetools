use anyhow::Result;
use celestedebugrc::DebugRC;

fn main() -> Result<()> {
    let debugrc = DebugRC::new();

    let res = debugrc.list_mods()?;
    dbg!(res);

    Ok(())
}
