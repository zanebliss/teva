use std::{
    env::{self, set_current_dir},
    path::Path,
};

use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn sanity_check_with_rspec() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = tempdir()?;
    let setup_script = Path::new("tests/fixtures/sanity_check_with_rspec/setup.sh");
    Command::new(setup_script).arg(tmp_dir.path()).output()?;

    set_current_dir(tmp_dir.path())?;

    let mut cmd = assert_cmd::Command::cargo_bin("teva")?;
    let result = String::from_utf8(cmd.output()?.stdout)?;

    insta::with_settings!({ filters => vec![
        (r"\b\d*\.\d+\b", "[TIME]"),
        (r"\b[a-f0-9]{7}\b", "[SHA]")
    ]}, {
        insta::assert_snapshot!(result);
    });

    set_current_dir(Path::new(&env::var("CARGO_MANIFEST_DIR")?))?;

    Ok(())
}
