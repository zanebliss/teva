use colored::Colorize;
use std::env::args;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};

#[derive(Default)]
struct Commit {
    sha: String,
    message: String,
}

fn main() {
    let mut cached_files: Vec<String> = vec![];
    let args: Vec<String> = args().collect();
    let mut from_sha = "main";

    if args.len() > 1 {
        from_sha = &args[1];
    }

    let commits: Vec<Commit> = get_commits(from_sha.to_string());

    for commit_pair in commits.windows(2) {
        print!(
            "{} {} ",
            &commit_pair[1].sha.yellow(),
            &commit_pair[1].message
        );

        io::stdout().flush().expect("Failed to flush stdout");

        let changed_files = get_changed_files(&commit_pair[0].sha, &commit_pair[1].sha);

        if changed_files.is_empty() {
            print!("{}\n", "No test files".bright_blue().bold());
            continue;
        }

        for file in &changed_files {
            if !cached_files.contains(&file) {
                cached_files.push(file.to_string());
            }
        }

        checkout(&commit_pair[1].sha);

        run_tests(&cached_files);

        checkout(&"-".to_string());

        print!("\n");
    }
}

fn get_commits(from_sha: String) -> Vec<Commit> {
    let child = Command::new("git")
        .args(["log", &format!("{from_sha}^.."), "--reverse", "--format=%h %s"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("error");

    child
        .stdout
        .map(|stdout| {
            BufReader::new(stdout)
                .lines()
                .map(|line| build_commit(line))
                .collect()
        })
        .unwrap_or_default()
}

fn build_commit<E>(line: Result<String, E>) -> Commit {
    match line.unwrap_or_default().split_once(" ") {
        Some((sha, message)) => Commit {
            sha: sha.to_string(),
            message: message.to_string(),
        },
        None => Commit::default(),
    }
}

fn get_changed_files(sha_1: &String, sha_2: &String) -> Vec<String> {
    let child = Command::new("git")
        .args(["diff", "--name-only", &sha_1, &sha_2])
        .stdout(Stdio::piped())
        .spawn()
        .expect("error");

    child
        .stdout
        .map(|stdout| {
            BufReader::new(stdout)
                .lines()
                .map(|line| line.expect("error"))
                .filter(|line| line.ends_with("_spec.rb"))
                .collect()
        })
        .unwrap_or_default()
}

fn checkout(value: &String) {
    Command::new("git")
        .args(["checkout", &format!("{}", value)])
        .output()
        .expect("error");
}

fn run_tests(cached_files: &Vec<String>) {
    let test_runner_command = Command::new("bundle")
        .args(["exec", "rspec"])
        .args(cached_files)
        .output()
        .expect("error");

    if test_runner_command.status.code() == Some(1) {
        println!("{} ❌\n", "Failed!".bright_red().bold());
        println!("{}\n", "RSpec output:".bright_blue().bold());
        io::stdout().write_all(&test_runner_command.stdout).unwrap();
    } else {
        print!("{} ✅", "Success!".bright_green().bold());
    }
}
