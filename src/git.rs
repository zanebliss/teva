use std::io::{BufRead, BufReader, Error};
use std::process::{Command, Stdio};

pub const GIT: &str = "git";
pub const WORKTREE_DIR: &str = "teva-worktree";
pub const DEFAULT_FROM_SHA: &str = "main";

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

pub fn get_commits(from_sha: String) -> Result<Vec<Commit>, Error> {
    let child = Command::new(GIT)
        .args([
            Subcommand::Log.to_string(),
            &format!("{from_sha}^.."),
            "--reverse",
            "--format=%h %s",
        ])
        .stdout(Stdio::piped())
        .spawn()?;

    match child.stdout.map(|stdout| {
        BufReader::new(stdout)
            .lines()
            .map(|line| build_commit(line))
            .collect::<Vec<Commit>>()
    }) {
        Some(val) => Ok(val),
        None => {
            eprintln!("No commit returned?");
            Ok(Vec::new())
        }
    }
}

pub fn get_changed_files(sha_1: &String, sha_2: &String) -> Result<Vec<String>, Error> {
    let child = Command::new(GIT)
        .args([Subcommand::Diff.to_string(), "--name-only", &sha_1, &sha_2])
        .stdout(Stdio::piped())
        .spawn()?;

    let changed_files: Vec<String> = child
        .stdout
        .map(|stdout| {
            BufReader::new(stdout)
                .lines()
                .map(|line| line.expect("error"))
                .collect()
        })
        .unwrap_or_default();

    Ok(changed_files)
}

pub fn checkout(value: &String) -> Result<(), Error> {
    Command::new(GIT)
        .args([Subcommand::Checkout.to_string(), &format!("{}", value)])
        .output()?;

    Ok(())
}

pub fn create_worktree() -> Result<(), Error> {
    Command::new(GIT)
        .args([
            Subcommand::Worktree.to_string(),
            "add",
            "-d",
            &format!("/tmp/{WORKTREE_DIR}"),
        ])
        .output()?;

    Ok(())
}

pub fn delete_worktree() -> Result<(), Error> {
    Command::new(GIT)
        .args([
            Subcommand::Worktree.to_string(),
            "remove",
            &format!("/tmp/{WORKTREE_DIR}"),
        ])
        .output()?;

    Ok(())
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
