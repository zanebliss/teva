use std::{env::{self, set_current_dir}, io::Error, path::Path};

pub fn restore_root_dir() -> Result<(), Error> {
    set_current_dir(Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()))?;

    Ok(())
}
