use std::path::Path;

use anyhow::{Context, Result};

use crate::{object::Object, repository::Repository};

pub fn run(file: &Path, write: bool) -> Result<()> {
    let repo = Repository::from_path(Path::new("."))?;
    let object = Object::blob_from_file(file).context("failed to hash file")?;

    let hash = if write {
        object.write_to_objects(&repo)?
    } else {
        object.write(std::io::sink())?
    };

    println!("{}", hash);

    Ok(())
}
