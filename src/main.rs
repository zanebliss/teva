use std::io::{self, Write};
use std::process;
use std::env::{current_dir, set_current_dir};
use clap::Parser;

mod git;
mod runners;
mod display;

fn main() {
    let cli = Cli::parse();

    display::print_logo();

    do_work(String::from(cli.from_sha.as_deref().unwrap_or("main")))
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    from_sha: Option<String>,
}

fn do_work(from_sha: String) {
    let mut cached_files: Vec<String> = vec![];
    let repo_dir = current_dir().unwrap();

    print!("\x1b[94m[GITAVS]\x1b[0m ⚙️ Setting up environment...");

    git::create_worktree();

    if set_current_dir(&format!("../{}", git::WORKTREE_DIR).to_string()).is_err() {
        eprintln!("Error, couldn't change to worktree directory");
        git::delete_worktree();
        process::exit(1);
    }

    runners::ruby::tests::rspec::setup_environment(repo_dir);

    print!(" Done ✔️\n");
    println!("\x1b[94m[GITAVS]\x1b[0m");

    let commits: Vec<git::Commit> = git::get_commits(from_sha);

    let mut i = 1;
    for commit_pair in commits.windows(2) {
        print!(
            "\x1b[94m[GITAVS]\x1b[0m \x1b[33m{}\x1b[0m {}",
            &commit_pair[1].sha, &commit_pair[1].message
        );
        print!(" ({i} of {})", commits.windows(2).len());

        io::stdout().flush().expect("Failed to flush stdout");

        let changed_files = git::get_changed_files(&commit_pair[0].sha, &commit_pair[1].sha);

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

        git::checkout(&commit_pair[1].sha);

        println!(
            "\n\x1b[94m[GITAVS]\x1b[0m Changed files: {}",
            changed_files.join(" ")
        );
        println!("\x1b[94m[GITAVS]\x1b[0m Running tests...");

        runners::ruby::tests::rspec::run(&cached_files);

        git::checkout(&"-".to_string());

        i += 1;
    }

    git::delete_worktree();
}

