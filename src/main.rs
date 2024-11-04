use std::sync::{
    atomic::{self, AtomicBool},
    Arc, Mutex,
};

use nix::sys::signal::{kill, Signal::SIGTERM};

fn main() -> revw::RevwResult<()> {
    let cli = revw::parse_args();
    let config = revw::Config::new()?;
    let head_child_pid = Arc::new(Mutex::new(None));
    let head_child_pid_clone = head_child_pid.clone();
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    ctrlc::set_handler(move || {
        println!("revw: Shutting down...");

        let head_child_pid = head_child_pid_clone.lock().unwrap().take();
        running_clone.store(false, atomic::Ordering::SeqCst);

        if let Some(pid) = head_child_pid {
            if let Err(e) = kill(pid, SIGTERM) {
                eprintln!("Error using kill for child pid: {}", e)
            }
        }
    })?;

    revw::run(
        cli.sha.as_str(),
        config,
        head_child_pid.clone(),
        running.clone(),
    )?;

    Ok(())
}
