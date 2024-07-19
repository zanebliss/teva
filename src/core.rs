use std::{io::Error, process, sync::atomic::AtomicBool};

use crate::{
    display,
    git::{self, Client},
    runners,
};

pub fn do_work(client: &Client, term: std::sync::Arc<AtomicBool>) -> Result<(), Error> {
    let cached_files: Vec<String> = vec![];
    let repo_dir = std::env::current_dir().unwrap();

    shutdown_if_no_work(client.commits.len());

    setup_environment(&client, repo_dir)?;

    for_each_commit_pair(client, cached_files, term, |cached_files| {
        let _ = runners::ruby::tests::rspec::run(&cached_files);
    })?;

    Ok(())
}

fn setup_environment(client: &Client, repo_dir: std::path::PathBuf) -> Result<(), Error> {
    print!("\x1b[94m[TEVA]\x1b[0m ⚙️ Setting up environment...");

    client.create_worktree()?;

    if std::env::set_current_dir(&format!("/tmp/{}", git::WORKTREE_DIR).to_string()).is_err() {
        eprintln!("Error, couldn't change to worktree directory");
        client.delete_worktree()?;
        std::process::exit(1);
    }

    runners::ruby::tests::rspec::setup_environment(repo_dir)?;

    print!(" Done ✔️\n");
    println!("\x1b[94m[TEVA]\x1b[0m");

    Ok(())
}

fn for_each_commit_pair<F>(
    client: &Client,
    mut cached_files: Vec<String>,
    term: std::sync::Arc<AtomicBool>,
    runner_fn: F,
) -> Result<(), Error>
where
    F: Fn(&Vec<String>),
{
    for (mut i, commit_pair) in client.commits.windows(2).enumerate() {
        if term.load(std::sync::atomic::Ordering::SeqCst) {
            break;
        }

        i += 1; // Start commit count at 1

        print!(
            "\x1b[94m[TEVA]\x1b[0m \x1b[33m{}\x1b[0m {}",
            &commit_pair[1].sha, &commit_pair[1].message
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
            "\n\x1b[94m[TEVA]\x1b[0m Changed files: {}",
            changed_files.join(" ")
        );
        println!("\x1b[94m[TEVA]\x1b[0m Running tests...");

        runner_fn(&cached_files);

        client.checkout(&"-".to_string())?;
    }

    Ok(())
}

pub fn cleanup(client: &Client) -> Result<(), Error> {
    client.delete_worktree()?;

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
