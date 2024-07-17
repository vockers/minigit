use std::{fs, path::Path};

use anyhow::{Context, Result};

use crate::repository;

// TODO: support detached HEAD
// TODO: branches in subdirectories
// TODO: write tests
pub fn run(create_branch: bool, branch: &str) -> Result<()> {
    let git_dir = Path::new(".git");

    if create_branch {
        // Get the commit of the current HEAD and write it to the new branch
        let head_ref = fs::read_to_string(git_dir.join("HEAD"))
            .context("read HEAD")?
            .trim_start_matches("ref: ")
            .trim()
            .to_string();
        let head_commit =
            fs::read_to_string(git_dir.join(head_ref)).context("read commit of ref")?;
        fs::write(git_dir.join("refs/heads").join(branch), head_commit).context("write branch")?;

        println!("Switched to a new branch '{}'", branch);
    } else {
        // Check if the branch exists
        let branch_exists = fs::read_dir(git_dir.join("refs/heads"))?
            .filter_map(Result::ok)
            .any(|entry| entry.file_name() == branch);

        if !branch_exists {
            anyhow::bail!("branch '{}' not found", branch);
        }

        println!("Switched to branch '{}'", branch);
    }

    repository::switch_branch(branch)?;

    return Ok(());
}
