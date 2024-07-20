use std::path::Path;

use anyhow::{Context, Result};

use crate::object::Object;

pub fn run(file: &Path, write: bool) -> Result<()> {
    let object = Object::blob_from_file(file).context("failed to hash file")?;

    let hash = if write {
        object.write_to_objects()?
    } else {
        object.write(std::io::sink())?
    };

    println!("{}", hash);

    Ok(())
}
