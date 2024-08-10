use colored::*;
use std::{
    error::Error,
    process::{self, Command},
    sync::atomic::AtomicBool,
};

use crate::{
    git::{self, Client},
    parser::Config,
    runners,
};

pub fn do_work(
    client: &Client,
    config: Config,
    term: std::sync::Arc<AtomicBool>,
) -> Result<(), Box<dyn Error>> {
    let cached_files: Vec<String> = vec![];

    shutdown_if_no_work(client.commits.len());

    setup_environment(&client, &config)?;

    for_each_commit_pair(client, cached_files, term, |cached_files| {
        runners::run(&config, &cached_files).unwrap();
    })?;

    Ok(())
}

fn setup_environment(client: &Client, config: &Config) -> Result<(), Box<dyn Error>> {
    print!("{} ⚙️ Setting up environment...", "[teva]".blue());

    client.create_worktree()?;

    if std::env::set_current_dir(&format!("/tmp/{}", git::WORKTREE_DIR).to_string()).is_err() {
        eprintln!("Error, couldn't change to worktree directory");
        client.delete_worktree()?;
        std::process::exit(1);
    }

    runners::setup(config)?;

    print!("{} Done ✔️\n", "[teva]".blue());

    Ok(())
}

fn for_each_commit_pair<F>(
    client: &Client,
    mut cached_files: Vec<String>,
    term: std::sync::Arc<AtomicBool>,
    runner_fn: F,
) -> Result<(), Box<dyn Error>>
where
    F: Fn(&Vec<String>),
{
    for (mut i, commit_pair) in client.commits.windows(2).enumerate() {
        if term.load(std::sync::atomic::Ordering::SeqCst) {
            break;
        }

        i += 1; // Start commit count at 1

        print!(
            "{} {} {}",
            "[teva]".blue(),
            &commit_pair[1].sha.yellow(),
            &commit_pair[1].message
        );
        print!(" ({i} of {})", client.commits.windows(2).len());

        let changed_files = client.get_changed_files(&commit_pair[0].sha, &commit_pair[1].sha);

        cached_files.extend(
            changed_files
                .iter()
                .filter(|file| !cached_files.contains(file))
                .map(|file| file.to_string())
                .collect::<Vec<String>>(),
        );

        client.checkout(&commit_pair[1].sha)?;

        println!(
            "\n{} Changed files: {}",
            "[teva]".blue(),
            changed_files.join(" "),
        );
        println!("{} Running tests...", "[teva]".blue());

        runner_fn(&cached_files);

        client.checkout(&"-".to_string())?;
    }

    Ok(())
}

pub fn cleanup(client: &Client) -> Result<(), Box<dyn Error>> {
    client.delete_worktree()?;

    Command::new("rm")
        .args(["-rf", "/tmp/teva-worktree"])
        .output()?;

    Ok(())
}

fn shutdown_if_no_work(commit_len: usize) {
    if commit_len > 1 {
        return;
    }

    println!("Number of commits from main to HEAD is 1 or less");
    println!("Try checking out to a branch with more commits, or use `teva --commit <commit_sha_or_branch>`");
    process::exit(0)
}
