use clap::Parser;

mod git_client;
mod runners;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    from_sha: Option<String>,
}

fn main() {
    let cached_files: Vec<String> = vec![];
    let cli = Cli::parse();


    git_client::do_work(
        String::from(cli.from_sha.as_deref().unwrap_or("main")),
        cached_files,
    )
}
