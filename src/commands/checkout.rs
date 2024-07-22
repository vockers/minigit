use std::path::Path;

use anyhow::Result;

use crate::repository::Repository;

// TODO: support detached HEAD
// TODO: branches in subdirectories
// TODO: write tests
pub fn run(create_branch: bool, branch: &str) -> Result<()> {
    let repo = Repository::from_path(Path::new("."))?;
    if create_branch {
        repo.create_branch(branch)?;
    }

    repo.switch_branch(branch)?;

    println!("Switched to branch '{}'", branch);

    return Ok(());
}
