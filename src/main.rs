use clap::Parser;
use std::{
    io::Error,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

mod core;
mod display;
mod git;
mod runners;

fn main() -> Result<(), Error> {
    let term = Arc::new(AtomicBool::new(false));
    let cli = Cli::parse();

    display::print_logo();

    signal_hook::flag::register(signal_hook::consts::SIGINT, term.clone())?;

    while !term.load(Ordering::SeqCst) {
        core::do_work(
            String::from(cli.sha.as_deref().unwrap_or(git::DEFAULT_FROM_SHA)),
            term,
        );

        // break after first iteration because work is not performed in a continuous loop
        break;
    }

    core::cleanup();

    Ok(())
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    sha: Option<String>,
}
