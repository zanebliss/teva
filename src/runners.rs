pub mod ruby {
    const BUNDLE: &str = "bundle";
    const EXEC: &str = "exec";

    pub mod tests {
        pub mod rspec {
            use std::path::{Path, PathBuf};
            use std::process;
            use std::process::Command;

            use crate::runners::node::{self, PACKAGE_JSON};
            use crate::runners::ruby::{BUNDLE, EXEC};

            const RSPEC: &str = "rspec";

            // Not every rails app will need this, but if sprockets attempts
            // to load JS assets in a test it will fail
            pub fn setup_environment(repo_dir: PathBuf) {
                if !repo_dir.join(PACKAGE_JSON).exists() {
                    return;
                }

                node::setup_environment(repo_dir);
            }

            pub fn run(cached_files: &Vec<String>) {
                let runnable_files = cached_files.iter().filter(|file| Path::new(file).exists());

                let child = match Command::new(BUNDLE)
                    .args([EXEC, RSPEC])
                    .args(runnable_files)
                    .spawn()
                {
                    Ok(child) => child,
                    Err(err) => {
                        eprintln!("Error running bundle exec rspec: {}", err);
                        process::exit(1)
                    }
                };

                child.wait_with_output().unwrap();
            }
        }
    }
}

pub mod node {
    use std::{
        env::current_dir,
        os::unix::fs::symlink,
        path::PathBuf,
        process::{self, Command},
    };

    const NODE_MODULES: &str = "node_modules";
    const YARN: &str = "yarn";
    pub const PACKAGE_JSON: &str = "package.json";

    pub fn setup_environment(repo_dir: PathBuf) {
        symlink_node_modules(repo_dir);

        _ = match Command::new(YARN).output() {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Failed to run npm: {}", err);
                process::exit(1);
            }
        }
    }

    fn symlink_node_modules(repo_dir: PathBuf) {
        let current_dir = match current_dir() {
            Ok(dir) => dir,
            Err(err) => {
                eprintln!("Error getting current_dir: {}", err);
                process::exit(1)
            }
        };

        _ = symlink(
            format!("{}/{NODE_MODULES}", repo_dir.display()),
            format!("{}/{NODE_MODULES}", current_dir.display()),
        )
    }
}
