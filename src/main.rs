use clap::Parser;
use std::io::Error;

mod core;
mod display;
mod git;
mod runners;

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    display::print_logo();

    core::do_work(String::from(
        cli.from_sha.as_deref().unwrap_or(git::DEFAULT_FROM_SHA),
    ));

    Ok(())
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    from_sha: Option<String>,
}
