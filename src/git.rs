use std::fs::File;
use std::io::{Error, Write};
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
            "log",
            &format!("{}^..", self.root_commit),
            "--reverse",
            "--format=%h %s",
        ]);

        self.transform_stream(output.stdout)
            .into_iter()
            .map(|line| Commit::build(line))
            .collect()
    }

    pub fn get_changed_files(&self, sha_1: &String, sha_2: &String) -> Vec<String> {
        let output = self.execute_command(vec!["diff", "--name-only", &sha_1, &sha_2]);

        self.transform_stream(output.stdout)
    }

    pub fn checkout(&self, value: &String) -> Result<(), Error> {
        self.execute_command(vec!["checkout", &format!("{}", value)]);

        Ok(())
    }

    pub fn create_worktree(&self) -> Result<(), Error> {
        self.execute_command(vec![
            "worktree",
            "add",
            "-d",
            &format!("/tmp/{WORKTREE_DIR}"),
        ]);

        Ok(())
    }

    pub fn delete_worktree(&self) -> Result<(), Error> {
        self.execute_command(vec!["worktree", "remove", &format!("/tmp/{WORKTREE_DIR}")]);

        Ok(())
    }

    fn transform_stream(&self, stdout: Vec<u8>) -> Vec<String> {
        String::from_utf8(stdout)
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

    fn execute_command(&self, args: Vec<&str>) -> Output {
        match Command::new("git").args(args).output() {
            Ok(output) => { 
                let mut dump = File::create("/tmp/teva.dump").unwrap();
                let _ = dump.write_all(&output.stderr);
                output
            }
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
        if let Some((sha, message)) = line.split_once(" ") {
            Commit {
                sha: sha.to_string(),
                message: message.to_string(),
            }
        } else {
            Commit::default()
        }
    }
}
