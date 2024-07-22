use std::path::Path;

use anyhow::{Context, Result};

use crate::{object, repository::Repository};

// TODO: write tests
pub fn run(message: &str) -> Result<()> {
    let repo = Repository::from_path(Path::new("."))?;
    let ref_path = repo.get_head()?;
    let parent_hash = repo.get_ref(&ref_path).ok();

    let tree_hash = object::write_tree(repo.get_root(), &repo).context("write tree")?;
    let commit_hash = object::write_commit(&tree_hash, parent_hash.as_deref(), message, &repo)
        .context("commit tree")?;

    repo.set_ref(&ref_path, &commit_hash).context("set HEAD")?;

    println!("{commit_hash}");

    Ok(())
}
