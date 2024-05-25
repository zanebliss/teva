use clap::Parser;

mod git;
mod runners;
mod display;
mod core;

fn main() {
    let cli = Cli::parse();

    display::print_logo();

    core::do_work(String::from(cli.from_sha.as_deref().unwrap_or("main")));
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    from_sha: Option<String>,
}

