use anyhow::Result;

use crate::{object, repository::Repository};

/// Create a new commit object.
pub fn run(tree_hash: &str, parent_hash: Option<&str>, message: &str) -> Result<()> {
    let repo = Repository::from_path(".")?;
    let hash = object::write_commit(tree_hash, parent_hash, message, &repo)?;

    println!("{}", hash);

    Ok(())
}
