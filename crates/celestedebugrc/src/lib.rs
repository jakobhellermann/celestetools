use anyhow::{bail, Context, Result};
use std::{
    path::{Path, PathBuf},
    thread::sleep,
    time::Duration,
};

use ureq::Request;

const PORT: u16 = 32270;

#[derive(Clone)]
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

pub struct PlayTasProgress<'a> {
    pub origin: Option<&'a str>,
    pub current_frame: &'a str,
    pub total_frames: &'a str,
}
impl DebugRC {
    pub fn run_tases_fastforward(
        &self,
        tas_files: &[PathBuf],
        speedup: f32,
        mut progress: impl FnMut(PlayTasProgress),
    ) -> Result<()> {
        let enforce_legal = tas_files.iter().fold(false, |acc, file| {
            let content = std::fs::read_to_string(&file).unwrap_or_default();
            acc || content.contains("EnforceLegal") || content.contains("EnforceMaingame")
        });

        if enforce_legal {
            eprintln!("File contains EnforceLegal, falling back to running TASes one by one");
        }

        let tmp_files = if enforce_legal {
            tas_files
                .iter()
                .map(|file| {
                    let name = file.file_name().unwrap().to_str().unwrap();
                    let file = file.to_str().unwrap();
                    (format!("Read,{file}\n***{speedup}"), Some(name))
                })
                .collect()
        } else {
            let mut temp_content = tas_files
                .iter()
                .map(|path| format!("Read,{}\n", path.to_str().unwrap()))
                .collect::<String>();
            temp_content.push_str("\n***");
            temp_content.push_str(&speedup.to_string());
            vec![(temp_content, None)]
        };

        for (content, origin) in tmp_files {
            let path = std::env::temp_dir().join("tmp.tas");

            std::fs::write(&path, content)?;
            self.play_tas_sync(&path, |info| {
                let current_frame = find_info(info, "CurrentFrame: ");
                let total_frames = find_info(info, "TotalFrames: ");
                progress(PlayTasProgress {
                    origin,
                    current_frame,
                    total_frames,
                });
            })
            .context("Could not play TAS. Is Celeste running?")?;
            std::fs::remove_file(&path)?;
        }

        Ok(())
    }
}

fn find_info<'a>(str: &'a str, prop: &str) -> &'a str {
    let Some(i) = str.find(prop) else { return "" };
    let str = &str[i + prop.len()..];

    let idx_newline = str.find("<br").unwrap_or(str.len());
    &str[..idx_newline]
}
