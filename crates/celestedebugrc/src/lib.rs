use anyhow::{ensure, Context, Result};
use std::{fmt::Write, path::Path, thread::sleep, time::Duration};

use ureq::Request;

const PORT: u16 = 32270;

#[derive(Clone)]
pub struct DebugRC {
    agent: ureq::Agent,
}

impl Default for DebugRC {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugRC {
    pub fn new() -> Self {
        DebugRC {
            agent: ureq::AgentBuilder::new()
                .timeout_connect(Duration::from_millis(100))
                .build(),
        }
    }

    fn get_request(&self, path: &str) -> Request {
        let url = format!("http://localhost:{PORT}/{path}");
        self.agent.request("GET", &url)
    }

    pub fn get(&self, path: &str) -> Result<()> {
        self.get_request(path).call()?;
        Ok(())
    }

    pub fn respawn(&self) -> Result<()> {
        self.get_request("respawn").call()?;
        Ok(())
    }

    pub fn play_tas(&self, file: impl AsRef<Path>) -> Result<()> {
        let file = file.as_ref().to_str().unwrap();
        self.get_request("tas/playtas")
            .query("filePath", file)
            .call()?
            .into_string()?;
        Ok(())
    }

    pub fn console(&self, command: &str) -> Result<()> {
        self.get_request("console")
            .query("command", command)
            .call()?;
        Ok(())
    }

    pub fn play_tas_sync(
        &self,
        file: impl AsRef<Path>,
        mut progress: impl FnMut(&str),
    ) -> Result<()> {
        self.play_tas(file)?;

        let start_timeout = Duration::from_secs_f32(0.1);
        let run_timeout = Duration::from_secs_f32(0.1);

        std::thread::sleep(start_timeout);
        /*let mut i_start = 0;
        loop {
            if self.tas_running(&mut |_| {})? {
                break;
            }

            sleep(start_timeout);
            i_start += 1;

            if i_start > 10 {
                bail!("didn't start tas in {:?}", start_timeout * i_start);
            }
        }*/

        let mut _i_run = 0;
        while self.tas_running(&mut progress)? {
            sleep(run_timeout);

            _i_run += 1;
        }

        Ok(())
    }

    fn tas_running(&self, progress: &mut impl FnMut(&str)) -> Result<bool> {
        let status = self.get_request("tas/info").call()?.into_string()?;
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
        let status = self.get_request("tas/info").call()?.into_string()?;
        Ok(status)
    }

    pub fn send_tas_keybind(&self, id: &str) -> Result<String> {
        let status = self
            .get_request("tas/sendhotkey")
            .query("action", "press")
            .query("id", id)
            .call()?
            .into_string()?;
        Ok(status)
    }
}

pub struct PlayTasProgress<'a> {
    pub origin: Option<&'a str>,
    pub current_frame: &'a str,
    pub total_frames: &'a str,
    pub current_file: usize,
    pub total_files: usize,
}
impl DebugRC {
    pub fn run_tases_fastforward(
        &self,
        tas_files: &[(impl AsRef<Path>, String, (String, String))],
        speedup: f32,
        mut run_as_merged_file: bool,
        mut progress: impl FnMut(PlayTasProgress),
    ) -> Result<()> {
        ensure!(!tas_files.is_empty(), "Tried to run zero TAS files");

        if run_as_merged_file {
            let enforce_legal = tas_files.iter().fold(false, |acc, (file, _, _)| {
                let content = std::fs::read_to_string(file).unwrap_or_default();
                acc || content.contains("EnforceLegal") || content.contains("EnforceMaingame")
            });

            if enforce_legal {
                eprintln!("File contains EnforceLegal, falling back to running TASes one by one");
                run_as_merged_file = false;
            }
        }
        let tmp_files = if run_as_merged_file {
            let mut temp_content = tas_files.iter().fold(
                String::new(),
                |mut acc, (path, _, (decorate_begin, decorate_end))| {
                    let _ = writeln!(
                        &mut acc,
                        "{decorate_begin}\nRead,{}\n{decorate_end}\n",
                        path.as_ref().to_str().unwrap()
                    );
                    acc
                },
            );
            temp_content.push_str("\n***");
            temp_content.push_str(&speedup.to_string());
            vec![(temp_content, None)]
        } else {
            tas_files
                .iter()
                .map(|(file, name, (decorate_begin, decorate_end))| {
                    let file = file.as_ref().to_str().unwrap();
                    (
                        format!("{decorate_begin}\nRead,{file}\n{decorate_end}\n***{speedup}"),
                        Some(name.as_str()),
                    )
                })
                .collect()
        };

        let total_files = tmp_files.len();
        for (i, (content, origin)) in tmp_files.into_iter().enumerate() {
            let path = std::env::temp_dir().join("tmp.tas");

            std::fs::write(&path, content)?;
            self.play_tas_sync(&path, |info| {
                let current_frame = find_info(info, "CurrentFrame: ");
                let total_frames = find_info(info, "TotalFrames: ");
                progress(PlayTasProgress {
                    origin,
                    current_frame,
                    total_frames,
                    total_files,
                    current_file: i,
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
