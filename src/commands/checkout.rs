use anyhow::Result;

use crate::repository;

// TODO: support detached HEAD
// TODO: branches in subdirectories
// TODO: write tests
pub fn run(create_branch: bool, branch: &str) -> Result<()> {
    if create_branch {
        repository::create_branch(branch)?;
    }

    repository::switch_branch(branch)?;

    println!("Switched to branch '{}'", branch);

    return Ok(());
}
