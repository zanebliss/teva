use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use colored::Colorize;

fn main() {
    struct Commit {
        sha: String,
        message: String,
    }

    let child = Command::new("git")
        .args(["log", "main^..", "--reverse", "--format=%h %s"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("error");

    let shas: Vec<Commit> = child
        .stdout
        .map(|stdout| {
            BufReader::new(stdout)
                .lines()
                .map(|line| {
                    if let Some((sha, message)) = line.ok().unwrap_or_default().split_once(" ") {
                        Commit {
                            sha: sha.to_string(),
                            message: message.to_string(),
                        }
                    } else {
                        Commit {
                            sha: "".to_string(),
                            message: "".to_string(),
                        }
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    for commit_pair in shas.windows(2) {
        print!("{} {} ", &commit_pair[1].sha.yellow(), &commit_pair[1].message);

        io::stdout().flush().expect("Failed to flush stdout");

        let child = Command::new("git")
            .args([
                "diff",
                "--name-only",
                &commit_pair[0].sha,
                &commit_pair[1].sha,
            ])
            .stdout(Stdio::piped())
            .spawn()
            .expect("error");

        let changed_files: Vec<String> = child
            .stdout
            .map(|stdout| {
                BufReader::new(stdout)
                    .lines()
                    .map(|line| line.expect("error"))
                    .filter(|line| line.ends_with("_spec.rb"))
                    .collect()
            })
            .unwrap_or_default();

        if changed_files.is_empty() {
            print!("{}\n", "No test files".bright_blue().bold());
            continue;
        }

        Command::new("git")
            .args(["checkout", &format!("{}", commit_pair[1].sha)])
            .output()
            .expect("error");

        let test_runner_command = Command::new("bundle")
            .args(["exec", "rspec"])
            .args(&changed_files)
            .output()
            .expect("error");

        if test_runner_command.status.code() == Some(1) {
            println!("{} ❌\n", "Failed!".bright_red().bold());
            println!("{}\n", "RSpec output:".bright_blue().bold());
            io::stdout().write_all(&test_runner_command.stdout).unwrap();

        } else {
            print!("{} ✅", "Success!".bright_green().bold());
        }

        Command::new("git")
            .args(["checkout", "-"])
            .output()
            .expect("error");

        print!("\n");
    }
}
