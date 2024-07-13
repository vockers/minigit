use std::{fs, path::Path};

use anyhow::{Context, Result};

use crate::object;

// TODO: write tests
pub fn run(message: &str) -> Result<()> {
    let head_ref = fs::read_to_string(".git/HEAD").context("read HEAD")?;
    let head_ref = head_ref
        .strip_prefix("ref: ")
        .context("can't commit onto a detached HEAD")?
        .trim();

    let head_ref_path = Path::new(".git").join(head_ref);
    let parent_hash = if head_ref_path.exists() {
        Some(
            fs::read_to_string(&head_ref_path)
                .context("read HEAD ref")?
                .trim()
                .to_string(),
        )
    } else {
        None
    };

    let tree_hash = object::write_tree(Path::new(".")).context("write tree")?;
    let commit_hash =
        object::write_commit(&tree_hash, parent_hash.as_deref(), message).context("commit tree")?;

    fs::write(head_ref_path, &commit_hash)
        .context(format!("update HEAD to reference: {commit_hash}"))?;

    println!("{commit_hash}");

    Ok(())
}
