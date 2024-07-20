use std::{fs, path::Path};

use anyhow::{Context, Result};

use crate::error::Error;

pub fn init(directory: &Path) -> Result<()> {
    let git_dir = directory.join(".git");
    fs::create_dir_all(git_dir.join("objects"))?;
    fs::create_dir_all(git_dir.join("refs"))?;
    fs::write(git_dir.join("HEAD"), "ref: refs/heads/main\n")?;

    Ok(())
}

pub fn create_branch(branch: &str) -> Result<()> {
    let git_dir = Path::new(".git");

    if branch_exists(branch)? {
        Err(Error::BranchAlreadyExists(branch.to_string()))?;
    }

    // Get the commit of the current HEAD and write it to the new branch
    let head_ref = fs::read_to_string(git_dir.join("HEAD"))
        .context("read HEAD")?
        .trim_start_matches("ref: ")
        .trim()
        .to_string();
    let head_commit = fs::read_to_string(git_dir.join(head_ref)).context("read commit of ref")?;
    fs::write(git_dir.join("refs/heads").join(branch), head_commit).context("write branch")?;

    Ok(())
}

pub fn switch_branch(branch: &str) -> Result<()> {
    let git_dir = Path::new(".git");

    if !branch_exists(branch)? {
        Err(Error::BranchNotFound(branch.to_string()))?;
    }
    // Update HEAD to reference the new branch
    let branch_ref = format!("ref: refs/heads/{}\n", branch);
    fs::write(git_dir.join("HEAD"), branch_ref).context("write HEAD")?;

    Ok(())
}

pub fn branch_exists(branch: &str) -> Result<bool> {
    let git_dir = Path::new(".git");
    Ok(fs::read_dir(git_dir.join("refs/heads"))?
        .filter_map(Result::ok)
        .any(|entry| entry.file_name() == branch))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_initialize_repository() {
        let temp_dir = tempdir().unwrap();
        let temp_dir_path = temp_dir.path().to_path_buf();

        init(&temp_dir_path).unwrap();

        let git_dir = temp_dir_path.join(".git");
        assert!(git_dir.exists());
        assert!(git_dir.join("objects").exists());
        assert!(git_dir.join("refs").exists());
        assert!(git_dir.join("HEAD").exists());
        assert_eq!(
            fs::read_to_string(git_dir.join("HEAD")).unwrap(),
            "ref: refs/heads/main\n"
        );
    }
}
