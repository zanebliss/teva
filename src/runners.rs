pub mod tests {
    pub mod rspec {
        use std::process::Command;
        use std::io::{self, Write};

        pub fn run(cached_files: &Vec<String>) {
            let test_runner_command = Command::new("bundle")
                .args(["exec", "rspec"])
                .args(cached_files)
                .output()
                .expect("error");

            if test_runner_command.status.code() == Some(1) {
                println!("{} ❌\n", "Failed!");
                println!("{}\n", "RSpec output:");
                io::stdout().write_all(&test_runner_command.stdout).unwrap();
            } else {
                print!("{} ✅", "Success!");
            }
        }
    }
}
