use std::path::Path;

use anyhow::Result;

use crate::{object::Object, repository::Repository};

/// Compute the hash of a file and optionally write it to the objects directory.
pub fn run<P: AsRef<Path>>(file: P, write: bool) -> Result<()> {
    let repo = Repository::from_path(".")?;
    let object = Object::blob_from_file(file)?;

    let hash = if write {
        object.write_to_objects(&repo)?
    } else {
        object.write(std::io::sink())?
    };

    println!("{}", hash);

    Ok(())
}
