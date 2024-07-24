use std::path::Path;

use anyhow::Result;

use crate::{object::Object, repository::Repository};

/// Provide contents or details of repository objects.
pub fn run(hash: &str) -> Result<()> {
    let repo = Repository::from_path(Path::new("."))?;
    let mut object = Object::read(hash, &repo)?;

    std::io::copy(&mut object.reader, &mut std::io::stdout())?;

    Ok(())
}
