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

    signal_hook::flag::register(signal_hook::consts::SIGINT, term.clone())?;

    while !term.load(Ordering::SeqCst) {
        match core::do_work(
            String::from(cli.sha.as_deref().unwrap_or(git::DEFAULT_FROM_SHA)),
            term,
        ) {
            Err(err) => {
                println!("\x1b[94m[TEVA]\x1b[0m Failed with error: {err}");
                println!("\x1b[94m[TEVA]\x1b[0m Exiting...");
            }
            _ => {}
        }

        // break after first iteration because work is not performed in a continuous loop
        break;
    }

    core::cleanup()?;

    Ok(())
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    sha: Option<String>,
}
