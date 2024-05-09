pub mod ruby {
    const BUNDLE: &str = "bundle";

    pub mod tests {
        pub mod rspec {
            use std::io::{self, Write};
            use std::process;
            use std::process::Command;

            use crate::runners::ruby::BUNDLE;

            pub fn run(cached_files: &Vec<String>) {
                let child = match Command::new(BUNDLE)
                    .args(["exec", "rspec"])
                    .args(cached_files)
                    .output()
                {
                    Ok(output) => output,
                    Err(err) => {
                        eprintln!("Error running bundle exec rspec: {}", err);
                        process::exit(1)
                    }
                };

                if child.status.code() == Some(1) {
                    println!("{} ❌\n", "Failed!");
                    println!("{}\n", "RSpec output:");
                    io::stdout().write_all(&child.stdout).unwrap();
                } else {
                    print!("{} ✅", "Success!");
                }
            }
        }
    }
}
