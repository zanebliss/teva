use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::process::{Child, Stdio};
use std::{fs, process::Command};

use crate::errors::{RevwError, RevwResult};

#[derive(Debug, Deserialize)]
pub struct Config {
    test: Test,
}

impl Config {
    pub fn new() -> RevwResult<Config> {
        parse_config()
    }

    pub fn file_matches_pattern(&self, file_name: &str) -> bool {
        file_name.contains(&self.test.pattern)
    }

    pub fn setup_environment(&self) -> RevwResult<()> {
        print!("[revw] ⚙️ Setting up environment...");

        if let Some(setup) = &self.test.setup {
            for (i, step) in setup.steps.iter().enumerate() {
                println!(
                    "\n[revw] Step ({} of {}) `{}`",
                    i + 1,
                    setup.steps.len(),
                    step.name
                );

                Command::new(&step.command)
                    .args(step.args.as_deref().unwrap_or_default())
                    .spawn()?
                    .wait_with_output()?;
            }
        }

        println!(" Done ✔️");

        Ok(())
    }

    pub fn spawn_test_runner(
        &self,
        worktree_path: &str,
        cached_path_bufs: &[PathBuf],
    ) -> RevwResult<Child> {
        let run_opts = self.test.run.as_ref().unwrap();

        let child = Command::new(format!("/tmp/{}/{}", worktree_path, run_opts.command))
            .args(run_opts.args.as_deref().unwrap_or_default())
            .args(cached_path_bufs.iter().map(|pb| pb.to_str().unwrap()))
            .stderr(Stdio::null())
            .spawn()?;

        Ok(child)
    }
}

fn parse_config() -> RevwResult<Config> {
    let path = Path::new(".revw.toml").to_path_buf();

    let contents = fs::read_to_string(path)?;

    let config = toml::from_str(contents.as_str())?;

    validate_keys(&config)?;

    Ok(config)
}

fn validate_keys(config: &Config) -> RevwResult<()> {
    if let Some(run) = &config.test.run {
        if !Path::new(&run.command).exists() {
            return Err(RevwError::InvalidPath);
        }

        Ok(())
    } else {
        Err(RevwError::MissingConfigKey)
    }
}

#[derive(Debug, Deserialize)]
struct Test {
    pattern: String,
    setup: Option<Setup>,
    run: Option<Run>,
}

#[derive(Debug, Deserialize)]
struct Setup {
    steps: Vec<Step>,
}

#[derive(Debug, Deserialize)]
struct Run {
    command: String,
    args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
struct Step {
    name: String,
    command: String,
    args: Option<Vec<String>>,
}
