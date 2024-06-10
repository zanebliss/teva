pub mod ruby {
    const BUNDLE: &str = "bundle";
    const EXEC: &str = "exec";

    pub mod tests {
        pub mod rspec {
            use std::io::Error;
            use std::path::{Path, PathBuf};
            use std::process::{Command, Stdio};

            use crate::runners::node::{self, PACKAGE_JSON};
            use crate::runners::ruby::{BUNDLE, EXEC};

            const RSPEC: &str = "rspec";

            // Not every rails app will need this, but if sprockets attempts
            // to load JS assets in a test it will fail
            pub fn setup_environment(repo_dir: PathBuf) -> Result<(), Error> {
                if !repo_dir.join(PACKAGE_JSON).exists() {
                    return Ok(());
                }

                node::setup_environment(repo_dir)?;

                Ok(())
            }

            pub fn run(cached_files: &Vec<String>) -> Result<(), Error> {
                let runnable_files = cached_files
                    .iter()
                    .map(|file| file.clone())
                    .filter(|file| Path::new(file).exists() && file.ends_with("_spec.rb"))
                    .collect::<Vec<String>>();


                Command::new(BUNDLE)
                    .args([EXEC, RSPEC])
                    .args(&runnable_files)
                    .stderr(Stdio::null())
                    .spawn()
                    .unwrap()
                    .wait_with_output()?;

                Ok(())
            }
        }
    }
}

pub mod node {
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
