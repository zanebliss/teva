use clap::Parser;

mod git_client;
mod runners;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    from_sha: Option<String>,
}

fn print_logo() {
    println!(
        r"
      (_)  _
  ____ _ _| |_ _____ _   _ ___
 / _  | (_   _|____ | | | /___)
( (_| | | | |_/ ___ |\ V /___ |
 \___ |_|  \__)_____| \_/(___/
(_____|
             "
    )
}

fn main() {
    let cached_files: Vec<String> = vec![];
    let cli = Cli::parse();

    print_logo();

    git_client::do_work(
        String::from(cli.from_sha.as_deref().unwrap_or("main")),
        cached_files,
    )
}
