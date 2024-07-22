use std::path::Path;

use anyhow::Result;

use crate::{object, repository::Repository};

pub fn run() -> Result<()> {
    let repo = Repository::from_path(Path::new("."))?;
    let hash = object::write_tree(Path::new("."), &repo)?;

    println!("{}", hash);

    Ok(())
}
