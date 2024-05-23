use std::env::{current_dir, set_current_dir};
use std::io::{self, BufRead, BufReader, Write};
use std::process::{self, Command, Stdio};

use crate::runners;

const GIT: &str = "git";
const WORKTREE_DIR: &str = "gitavs-worktree";

#[derive(Default)]
struct Commit {
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

pub fn do_work(from_sha: String) {
    let mut cached_files: Vec<String> = vec![];
    let repo_dir = current_dir().unwrap();

    print!("\x1b[94m[GITAVS]\x1b[0m ⚙️ Setting up environment...");

    create_worktree();

    if set_current_dir(&format!("../{WORKTREE_DIR}").to_string()).is_err() {
        eprintln!("Error, couldn't change to worktree directory");
        delete_worktree();
        process::exit(1);
    }

    runners::ruby::tests::rspec::setup_environment(repo_dir);

    print!(" Done ✔️\n");
    println!("\x1b[94m[GITAVS]\x1b[0m");

    let commits: Vec<Commit> = get_commits(from_sha);

    let mut i = 1;
    for commit_pair in commits.windows(2) {
        print!(
            "\x1b[94m[GITAVS]\x1b[0m \x1b[33m{}\x1b[0m {}",
            &commit_pair[1].sha, &commit_pair[1].message
        );
        print!(" ({i} of {})", commits.windows(2).len());

        io::stdout().flush().expect("Failed to flush stdout");

        let changed_files = get_changed_files(&commit_pair[0].sha, &commit_pair[1].sha);

        if changed_files.is_empty() {
            print!(" \x1b[2mNo test files\x1b[0m\n");

            i += 1;

            continue;
        }

        for file in &changed_files {
            if !cached_files.contains(&file) {
                cached_files.push(file.to_string());
            }
        }

        checkout(&commit_pair[1].sha);

        println!(
            "\n\x1b[94m[GITAVS]\x1b[0m Changed files: {}",
            changed_files.join(" ")
        );
        println!("\x1b[94m[GITAVS]\x1b[0m Running tests...");

        runners::ruby::tests::rspec::run(&cached_files);

        checkout(&"-".to_string());

        i += 1;
    }

    delete_worktree();
}

fn get_commits(from_sha: String) -> Vec<Commit> {
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

fn get_changed_files(sha_1: &String, sha_2: &String) -> Vec<String> {
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

fn checkout(value: &String) {
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

fn create_worktree() {
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

fn delete_worktree() {
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
