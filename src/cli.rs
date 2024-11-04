use std::env;

#[derive(Debug)]
pub struct Cli {
    pub sha: String,
}

impl Cli {
    fn new() -> Cli {
        Cli {
            sha: "main".to_owned(),
        }
    }
}

pub fn parse_args() -> Cli {
    let mut cli = Cli::new();
    let mut iter = env::args();
    iter.next(); // Discard program itself

    if iter.len() == 0 {
        return cli;
    }

    match iter.len() {
        1 => {
            handle_arg(iter.next().unwrap_or_default().as_str());

            cli
        }
        2 => {
            let flag = iter.next();
            let value = iter.next();

            let flag = get_flag(
                flag.unwrap_or_default().as_str(),
                value.unwrap_or_default().as_str(),
            );

            if let Some(flag) = flag {
                cli.sha = flag;
            }

            cli
        }
        _ => {
            print_help();

            cli
        }
    }
}

fn handle_arg(arg: &str) {
    match arg {
        "help" | "--help" | "-h" => {
            print_help();
        }
        "-v" | "--version" => {
            let version = format!("revward version {}", env!("CARGO_PKG_VERSION"));

            println!("{version}");
        }
        "--sha" => {
            println!("Error: provide <sha>.")
        }
        _ => {
            print_unknown_command(arg);
        }
    }
}

fn get_flag(flag: &str, value: &str) -> Option<String> {
    match flag {
        "--sha" => Some(value.to_owned()),
        _ => {
            print_unknown_command(flag);

            None
        }
    }
}

fn print_unknown_command(arg: &str) {
    let error = format!(
        "
revward: {} is not a revward command. See 'revward --help'.
    ",
        arg
    );

    println!("{error}");
}

fn print_help() {
    let help = "
Usage: revward [OPTIONS]

Optional Arguments:
  --sha <sha>   Start at a different commit sha then the sha pointed to by `main`.
  -h, --help    Show this help message and exit.
  -v --version  Show the version of the tool.
    ";

    println!("{help}");
}
