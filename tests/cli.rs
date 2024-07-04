use std::{
    env::set_current_dir,
    path::Path,
    process::Command,
};

use assert_cmd::Command as AssertCmd;
use common::restore_root_dir;
use tempfile::tempdir;

mod common;

// #[test]
// fn sanity_check_with_rspec() -> Result<(), Box<dyn std::error::Error>> {
//     // create a temp directory
//     // cd into it
//     // copy files from fixture directories into it
//     // execute the setup script
//     let tmp_dir = tempdir()?;
//     let path = Path::new("tests/fixtures/sanity_check_with_rspec/setup.sh");
//     Command::new(path).arg(tmp_dir.path()).output()?;
//     set_current_dir(tmp_dir)?;

//     let mut cmd = assert_cmd::Command::cargo_bin("teva")?;

//     insta::with_settings!({ filters => vec![
//         (r"\b\d*\.\d+\b", "[TIME]"),
//         (r"33m[a-z0-9]{7}", "[SHA]")
//     ]}, {
//         insta::assert_snapshot!(String::from_utf8(cmd.output().unwrap().stdout).unwrap());
//     });

//     restore_root_dir()?;

//     Ok(())
// }

#[test]
fn not_enough_commits() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = tempdir()?;
    let setup_script = Path::new("tests/fixtures/not_enough_commits/setup.sh");
    println!("{:?}", String::from_utf8(Command::new("ls").arg("tests/fixtures/not_enough_commits").output().unwrap().stdout));
    // Command::new(setup_script).arg(tmp_dir.path()).output()?;
    // set_current_dir(tmp_dir.path())?;

    let mut cmd = AssertCmd::cargo_bin("teva")?;

    insta::with_settings!({ filters => vec![
        (r"\b\d*\.\d+\b", "[TIME]"),
        (r"33m[a-z0-9]{7}", "[SHA]")
    ]}, {
        insta::assert_snapshot!(String::from_utf8(cmd.output().unwrap().stdout).unwrap());
    });

    restore_root_dir()?;

    Ok(())
}
