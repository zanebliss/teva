use std::io::Error;
use std::process::{self, Command, Output};

pub const WORKTREE_DIR: &str = "teva-worktree";
pub const DEFAULT_COMMIT: &str = "main";

pub struct Client {
    pub root_commit: String,
    pub commits: Vec<Commit>,
}

impl Client {
    pub fn new(root_commit: String) -> Self {
        let mut client = Client {
            root_commit,
            commits: vec![],
        };

        client.commits = client.get_commits();
        client
    }

    pub fn get_commits(&self) -> Vec<Commit> {
        let output = self.execute_command(vec![
            Subcommand::Log.to_string(),
            &format!("{}^..", self.root_commit),
            "--reverse",
            "--format=%h %s",
        ]);

        String::from_utf8(output.stdout)
            .unwrap()
            .split('\n')
            .filter_map(|line| {
                if line.is_empty() {
                    None
                } else {
                    Some(Commit::build(line.to_string()))
                }
            })
            .collect::<Vec<Commit>>()
    }

    pub fn get_changed_files(&self, sha_1: &String, sha_2: &String) -> Vec<String> {
        let output = self.execute_command(vec![
            Subcommand::Diff.to_string(),
            "--name-only",
            &sha_1,
            &sha_2,
        ]);

        String::from_utf8(output.stdout)
            .unwrap()
            .split('\n')
            .filter_map(|line| {
                if line.is_empty() {
                    None
                } else {
                    Some(line.to_string())
                }
            })
            .collect()
    }

    pub fn checkout(&self, value: &String) -> Result<(), Error> {
        self.execute_command(vec![
            Subcommand::Checkout.to_string(),
            &format!("{}", value),
        ]);

        Ok(())
    }

    pub fn create_worktree(&self) -> Result<(), Error> {
        self.execute_command(vec![
            Subcommand::Worktree.to_string(),
            "add",
            "-d",
            &format!("/tmp/{WORKTREE_DIR}"),
        ]);

        Ok(())
    }

    pub fn delete_worktree(&self) -> Result<(), Error> {
        self.execute_command(vec![
            Subcommand::Worktree.to_string(),
            "remove",
            &format!("/tmp/{WORKTREE_DIR}"),
        ]);

        Ok(())
    }

    fn execute_command(&self, args: Vec<&str>) -> Output {
        match Command::new("git").args(args).output() {
            Ok(output) => output,
            Err(err) => {
                eprint!("Error executing git command: {}", err);
                process::exit(1);
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct Commit {
    pub sha: String,
    pub message: String,
}

impl Commit {
    fn build(line: String) -> Commit {
        match line.split_once(" ") {
            Some((sha, message)) => Commit {
                sha: sha.to_string(),
                message: message.to_string(),
            },
            None => Commit::default(),
        }
    }
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
