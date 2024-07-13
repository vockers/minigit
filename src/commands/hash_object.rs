use std::path::Path;

use anyhow::Result;

use crate::object::Object;

pub fn run(file: &Path, write: bool) -> Result<()> {
    let object = Object::blob_from_file(file)?;

    let hash = if write {
        object.write_to_objects()?
    } else {
        object.write(std::io::sink())?
    };

    println!("{}", hash);

    Ok(())
}
