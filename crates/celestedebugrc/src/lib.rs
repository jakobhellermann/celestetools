use anyhow::{bail, Result};
use std::{path::Path, thread::sleep, time::Duration};

use ureq::Request;

const PORT: u16 = 32270;

pub struct DebugRC {
    agent: ureq::Agent,
}

impl DebugRC {
    pub fn new() -> Self {
        DebugRC {
            agent: ureq::Agent::new(),
        }
    }

    fn get(&self, path: &str) -> Request {
        let url = format!("http://localhost:{PORT}/{path}");
        self.agent.request("GET", &url)
    }

    pub fn respawn(&self) -> Result<()> {
        self.get("respawn").call()?;
        Ok(())
    }

    pub fn play_tas(&self, file: impl AsRef<Path>) -> Result<()> {
        let file = file.as_ref().to_str().unwrap();
        self.get("tas/playtas").query("filePath", file).call()?;
        Ok(())
    }

    pub fn console(&self, command: &str) -> Result<()> {
        self.get("console").query("command", command).call()?;
        Ok(())
    }

    pub fn play_tas_sync(
        &self,
        file: impl AsRef<Path>,
        mut progress: impl FnMut(&str),
    ) -> Result<()> {
        self.play_tas(file)?;

        /*self.get("tas/sendhotkey")
        .query("id", "FastForward")
        .query("action", "hold")
        .call()?;*/

        let start_timeout = Duration::from_secs_f32(0.3);
        let run_timeout = Duration::from_secs_f32(0.3);

        let mut i_start = 0;
        loop {
            if self.tas_running(&mut |_| {})? {
                break;
            }

            sleep(start_timeout);
            i_start += 1;

            if i_start > 10 {
                bail!("didn't start tas in {:?}", start_timeout * i_start);
            }
        }

        let mut _i_run = 0;
        while self.tas_running(&mut progress)? {
            sleep(run_timeout);

            _i_run += 1;
        }

        Ok(())
    }

    fn tas_running(&self, progress: &mut impl FnMut(&str)) -> Result<bool> {
        let status = self.get("tas/info").call()?.into_string()?;
        progress(&status);
        if status.contains("Running: False") {
            return Ok(false);
        }
        if status.contains("Running: True") {
            return Ok(true);
        }

        Err(anyhow::Error::msg("could not understand tas/info response"))
    }

    pub fn tas_info(&self) -> Result<String> {
        let status = self.get("tas/info").call()?.into_string()?;
        Ok(status)
    }
}
