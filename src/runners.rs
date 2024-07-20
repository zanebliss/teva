use std::{
    io::Error,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub struct Runner {
    pub runnable: Ruby,
}

pub enum Ruby {
    Rspec,
}

impl Ruby {
    fn program(&self) -> &str {
        match &self {
            Ruby::Rspec => "bundle",
        }
    }

    fn args(&self) -> Vec<&str> {
        match &self {
            Ruby::Rspec => vec!["exec", "rspec"],
        }
    }

    fn file_suffix(&self) -> &str {
        match &self {
            Ruby::Rspec => "_spec.rb",
        }
    }
}

impl Runner {
    // Not every rails app will need this, but if sprockets attempts
    // to load JS assets in a test it will fail
    pub fn setup_environment(&self, repo_dir: PathBuf) -> Result<(), Error> {
        if !repo_dir.join(node::PACKAGE_JSON).exists() {
            return Ok(());
        }

        node::setup_environment(repo_dir)?;

        Ok(())
    }

    pub fn run(&self, cached_files: &Vec<String>) -> Result<(), Error> {
        let runnable_files = cached_files
            .iter()
            .map(|file| file.clone())
            .filter(|file| Path::new(file).exists() && file.ends_with(self.runnable.file_suffix()))
            .collect::<Vec<String>>();

        if runnable_files.is_empty() {
            return Ok(());
        }

        self.execute(runnable_files)?;

        Ok(())
    }

    fn execute(&self, runnable_files: Vec<String>) -> Result<(), Error> {
        Command::new(self.runnable.program())
            .args(self.runnable.args())
            .args(runnable_files)
            .stderr(Stdio::null())
            .spawn()
            .unwrap()
            .wait_with_output()?;

        Ok(())
    }
}

mod node {
    use std::{
        env::current_dir, io::Error, os::unix::fs::symlink, path::PathBuf, process::Command,
    };

    const NODE_MODULES: &str = "node_modules";
    const YARN: &str = "yarn";
    pub const PACKAGE_JSON: &str = "package.json";

    pub fn setup_environment(repo_dir: PathBuf) -> Result<(), Error> {
        symlink_node_modules(repo_dir)?;

        Command::new(YARN).output()?;

        Ok(())
    }

    fn symlink_node_modules(repo_dir: PathBuf) -> Result<(), Error> {
        let current_dir = current_dir()?;

        symlink(
            format!("{}/{NODE_MODULES}", repo_dir.display()),
            format!("{}/{NODE_MODULES}", current_dir.display()),
        )?;

        Ok(())
    }
}
