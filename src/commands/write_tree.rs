use anyhow::Result;

use crate::{object, repository::Repository};

/// Create a tree object from the current index.
pub fn run() -> Result<()> {
    let repo = Repository::from_path(".")?;
    let hash = object::write_tree(".", &repo)?;

    println!("{}", hash);

    Ok(())
}
