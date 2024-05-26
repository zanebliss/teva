use crate::{git, runners};

pub fn do_work(from_sha: String) {
    let cached_files: Vec<String> = vec![];
    let repo_dir = std::env::current_dir().unwrap();

    setup_environment(repo_dir);

    let commits: Vec<git::Commit> = git::get_commits(from_sha);

    for_each_commit_pair(commits, cached_files, |cached_files| {
        runners::ruby::tests::rspec::run(&cached_files)
    });

    git::delete_worktree();
}

fn setup_environment(repo_dir: std::path::PathBuf) {
    print!("\x1b[94m[GITAVS]\x1b[0m ⚙️ Setting up environment...");

    git::create_worktree();

    if std::env::set_current_dir(&format!("../{}", git::WORKTREE_DIR).to_string()).is_err() {
        eprintln!("Error, couldn't change to worktree directory");
        git::delete_worktree();
        std::process::exit(1);
    }

    runners::ruby::tests::rspec::setup_environment(repo_dir);

    print!(" Done ✔️\n");
    println!("\x1b[94m[GITAVS]\x1b[0m");
}

fn for_each_commit_pair<F>(commits: Vec<git::Commit>, mut cached_files: Vec<String>, runner_fn: F)
where
    F: Fn(&Vec<String>),
{
    for (mut i, commit_pair) in commits.windows(2).enumerate() {
        i += 1; // Start commit count at 1

        print!(
            "\x1b[94m[GITAVS]\x1b[0m \x1b[33m{}\x1b[0m {}",
            &commit_pair[1].sha, &commit_pair[1].message
        );
        print!(" ({i} of {})", commits.windows(2).len());

        let changed_files = git::get_changed_files(&commit_pair[0].sha, &commit_pair[1].sha);

        if changed_files.is_empty() {
            print!(" \x1b[2mNo test files\x1b[0m\n");

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

        runner_fn(&cached_files);

        git::checkout(&"-".to_string());
    }
}
