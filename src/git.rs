use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

pub const GIT: &str = "git";
pub const WORKTREE_DIR: &str = "gitavs-worktree";

#[derive(Default)]
pub struct Commit {
    pub sha: String,
    pub message: String,
}

enum Subcommand {
    Diff,
    Log,
    Checkout,
    Worktree,
}

impl Subcommand {
    fn to_string(&self) -> &str {
        match self {
            Subcommand::Log => "log",
            Subcommand::Diff => "diff",
            Subcommand::Checkout => "checkout",
            Subcommand::Worktree => "worktree",
        }
    }
}

pub fn get_commits(from_sha: String) -> Vec<Commit> {
    let child = match Command::new(GIT)
        .args([
            Subcommand::Log.to_string(),
            &format!("{from_sha}^.."),
            "--reverse",
            "--format=%h %s",
        ])
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(err) => {
            eprintln!("Error spawning process: {}", err);
            std::process::exit(1);
        }
    };

    match child.stdout.map(|stdout| {
        BufReader::new(stdout)
            .lines()
            .map(|line| build_commit(line))
            .collect::<Vec<Commit>>()
    }) {
        Some(val) => val,
        None => {
            eprintln!("No commit returned?");
            Vec::new()
        }
    }
}

pub fn get_changed_files(sha_1: &String, sha_2: &String) -> Vec<String> {
    let child = match Command::new(GIT)
        .args([Subcommand::Diff.to_string(), "--name-only", &sha_1, &sha_2])
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(err) => {
            eprintln!("Error spawning process: {}", err);
            std::process::exit(1);
        }
    };

    let changed_files: Vec<String> = child
        .stdout
        .map(|stdout| {
            BufReader::new(stdout)
                .lines()
                .map(|line| line.expect("error"))
                .collect()
        })
        .unwrap_or_default();

    changed_files
}

pub fn checkout(value: &String) {
    match Command::new(GIT)
        .args([Subcommand::Checkout.to_string(), &format!("{}", value)])
        .output()
    {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Failed to switch: {}", err);
            std::process::exit(1);
        }
    }
}

pub fn create_worktree() {
    match Command::new(GIT)
        .args([
            Subcommand::Worktree.to_string(),
            "add",
            "-d",
            &format!("../{WORKTREE_DIR}"),
        ])
        .output()
    {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Failed to add worktree: {}", err);
            std::process::exit(1);
        }
    }
}

pub fn delete_worktree() {
    match Command::new(GIT)
        .args([
            Subcommand::Worktree.to_string(),
            "remove",
            &format!("../{WORKTREE_DIR}"),
        ])
        .output()
    {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Failed to remove worktree: {}", err);
            std::process::exit(1);
        }
    }
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
