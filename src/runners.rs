use std::{
    error::Error,
    io::{self, Write},
    path::Path,
    process::{Command, Stdio},
};

use colored::Colorize;

use crate::parser::Config;

pub fn setup(config: &Config) -> Result<(), Box<dyn Error>> {
    if let Some(setup) = &config.test.setup {
        let mut count = 1;

        for step in &setup.steps {
            println!(
                "\n{} Step ({} of {}) `{}`",
                "[teva]".blue(),
                count,
                setup.steps.len(),
                step.name
            );

            let output = Command::new(&step.command)
                .args(step.args.as_deref().unwrap_or(&[]))
                .output()?;

            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();

            count += 1;
        }
    }

    Ok(())
}

pub fn run(config: &Config, cached_files: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let runnable_files: Vec<_> = cached_files
        .iter()
        .filter(|file| Path::new(file).exists() && file.ends_with(config.test.pattern.as_str()))
        .collect();

    if runnable_files.is_empty() {
        return Ok(());
    }

    if let Some(run) = &config.test.run {
        for step in &run.steps {
            Command::new(&step.command)
                .args(step.args.as_deref().unwrap_or_default())
                .args(&runnable_files)
                .stderr(Stdio::null())
                .spawn()?
                .wait_with_output()?;
        }
    }

    Ok(())
}
