use clap::Parser;
use std::{io::Error, sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
}};

mod core;
mod display;
mod git;
mod runners;

fn main() -> Result<(), Error>{
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let cli = Cli::parse();

    display::print_logo();

    while running.load(Ordering::SeqCst) {
        core::do_work(
            String::from(cli.from_sha.as_deref().unwrap_or(git::DEFAULT_FROM_SHA)),
            running.clone(),
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
    from_sha: Option<String>,
}
