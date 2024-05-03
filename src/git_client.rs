use colored::Colorize;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};

const GIT: &str = "git";
const WORKTREE_DIR: &str = "gitavs-worktree";

use crate::runners::tests;

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

    create_worktree();

    match Command::new("cd").arg(format!("../{WORKTREE_DIR}")).output() {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Could not cd: {}", err);
            std::process::exit(-1);
        }
    }

    let commits: Vec<Commit> = get_commits(from_sha);

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

        tests::rspec::run(&cached_files);

        checkout(&"-".to_string());

        print!("\n");
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

    child
        .stdout
        .map(|stdout| {
            BufReader::new(stdout)
                .lines()
                .map(|line| line.expect("error"))
                .filter(|line| line.ends_with("_spec.rb") || line.ends_with("_test.rb"))
                .collect()
        })
        .unwrap_or_default()
}

fn checkout(value: &String) {
    match Command::new(GIT)
        .args([Subcommand::Checkout.to_string(), &format!("{}", value)])
        .output()
    {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Failed to switch: {}", err);
            std::process::exit(-1);
        }
    }
}

fn create_worktree() {
    match Command::new(GIT)
        .args([Subcommand::Worktree.to_string(), "add", "-d", &format!("../{WORKTREE_DIR}")])
        .output()
    {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Failed to add worktree: {}", err);
            std::process::exit(-1);
        }
    }
}

fn delete_worktree() {
    match Command::new(GIT)
        .args([Subcommand::Worktree.to_string(), "remove", &format!("../{WORKTREE_DIR}")])
        .output()
    {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Failed to remove worktree: {}", err);
            std::process::exit(-1);
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
