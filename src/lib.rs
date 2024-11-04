mod cli;
mod config;
mod errors;

use errors::RevwError;
use git2::{BranchType, Commit, Repository, Sort, Worktree, WorktreePruneOptions};
use nix::unistd::Pid;
use std::{
    env::temp_dir,
    path::PathBuf,
    sync::{
        atomic::{self, AtomicBool},
        Arc, Mutex,
    },
};

pub use cli::parse_args;
pub use config::Config;
pub use errors::RevwResult;

pub fn run(
    root_commit: &str,
    config: Config,
    head_child_pid: Arc<Mutex<Option<Pid>>>,
    running: Arc<AtomicBool>,
) -> RevwResult<()> {
    let repository = Repository::open_from_env()?;

    let count = ensure_enough_commits(root_commit, &repository)?;

    let revward_worktree_path = format!("revward-{}", repository.revparse_single("HEAD")?.id());

    ensure_no_worktree_and_branch(&repository, revward_worktree_path.as_str())?;

    config.setup_environment()?;

    let tmp_worktree = create_worktree(&repository)?;
    let tmp_repository = Repository::open_from_worktree(&tmp_worktree)?;
    let mut revwalk = tmp_repository.revwalk()?;

    revwalk.push_range(format!("{}..", root_commit).as_str())?;
    revwalk.set_sorting(Sort::REVERSE)?;

    let mut cached_path_bufs: Vec<PathBuf> = vec![];

    for (i, oid) in revwalk.enumerate() {
        if !running.load(atomic::Ordering::SeqCst) {
            break;
        }

        let commit = tmp_repository.find_commit(oid?)?;

        println!(
            "[revw] {} {} ({} of {count})",
            &commit.id().to_string()[..6],
            &commit.summary().unwrap_or("[No message]"),
            i + 1
        );

        let changed_files_path_bufs =
            get_changed_file_path_bufs(&commit, &tmp_repository, &config)?;

        changed_files_path_bufs.into_iter().for_each(|file| {
            if file.exists() {
                cached_path_bufs.push(file)
            }
        });

        tmp_repository.checkout_tree(commit.as_object(), None)?;
        let annotated_commit = tmp_repository.find_annotated_commit(commit.id())?;
        tmp_repository.set_head_detached_from_annotated(annotated_commit)?;

        run_tests(
            &revward_worktree_path,
            &cached_path_bufs,
            &config,
            head_child_pid.clone(),
        )?;
    }

    ensure_no_worktree_and_branch(&tmp_repository, &revward_worktree_path)?;

    Ok(())
}

fn run_tests(
    worktree_path: &str,
    cached_path_bufs: &[PathBuf],
    config: &Config,
    head_child_pid: Arc<Mutex<Option<Pid>>>,
) -> RevwResult<()> {
    let child = config.spawn_test_runner(worktree_path, cached_path_bufs)?;

    *head_child_pid.lock().unwrap() = Some(Pid::from_raw(child.id() as i32));

    child.wait_with_output()?;

    *head_child_pid.lock().unwrap() = None;

    Ok(())
}

fn ensure_enough_commits(root_commit: &str, repository: &Repository) -> RevwResult<i32> {
    let mut count: i32 = 0;

    let mut revwalk = repository.revwalk()?;
    revwalk.push_range(format!("{}..", root_commit).as_str())?;

    revwalk.for_each(|_| count += 1);

    if count.is_positive() {
        Ok(count)
    } else {
        Err(RevwError::NotEnoughCommits)
    }
}

fn ensure_no_worktree_and_branch(repository: &Repository, worktree_name: &str) -> RevwResult<()> {
    let worktree = match repository.find_worktree(worktree_name) {
        Ok(worktree) => worktree,
        Err(e) => {
            if e.message().contains("no error") {
                return Ok(());
            } else {
                return Err(RevwError::Git(e));
            }
        }
    };

    let mut prune_options = WorktreePruneOptions::new();
    prune_options.working_tree(true);
    prune_options.valid(true);

    worktree.prune(Some(&mut prune_options))?;

    let mut worktree_branch = repository.find_branch(worktree_name, BranchType::Local)?;

    worktree_branch.delete()?;

    Ok(())
}

fn create_worktree(repository: &Repository) -> RevwResult<Worktree> {
    let worktree_name = format!("revward-{}", repository.revparse_single("HEAD")?.id());

    Ok(repository.worktree(
        worktree_name.as_str(),
        &temp_dir().join(&worktree_name),
        None,
    )?)
}

fn get_changed_file_path_bufs(
    commit: &Commit,
    repository: &Repository,
    config: &Config,
) -> RevwResult<Vec<PathBuf>> {
    let current_tree = Some(commit.tree()?);
    let parent_tree = Some(commit.parent(0)?.tree()?);

    let diff = repository.diff_tree_to_tree(parent_tree.as_ref(), current_tree.as_ref(), None)?;

    let mut changed_file_path_bufs = vec![];

    for delta in diff.deltas() {
        if let Some(path) = delta.old_file().path() {
            if config.file_matches_pattern(path.to_str().unwrap_or_default()) {
                changed_file_path_bufs.push(path.into());
            }
        };
    }

    Ok(changed_file_path_bufs)
}
