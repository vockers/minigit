use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use crate::error::Error;

pub struct Repository {
    dir: PathBuf,
}

impl Repository {
    /// Returns a new Repository instance from the given path.
    pub fn from_path(path: &Path) -> Result<Self> {
        if !path.exists() {
            Err(Error::NotGitRepository)?;
        }

        Ok(Self {
            dir: path.to_path_buf(),
        })
    }

    /// Initializes a new Git repository at the given directory.
    pub fn init(directory: &Path) -> Result<Repository> {
        let git_dir = directory.join(".git");
        fs::create_dir_all(git_dir.join("objects"))?;
        fs::create_dir_all(git_dir.join("refs"))?;
        fs::write(git_dir.join("HEAD"), "ref: refs/heads/main\n")?;

        Ok(Repository {
            dir: directory.to_path_buf(),
        })
    }

    /// Creates a new branch with the given name.
    pub fn create_branch(&self, branch: &str) -> Result<()> {
        if self.branch_exists(branch)? {
            Err(Error::BranchAlreadyExists(branch.to_string()))?;
        }

        // Get the commit of the current HEAD and write it to the new branch
        let head_ref = fs::read_to_string(self.dir.join("HEAD"))
            .context("read HEAD")?
            .trim_start_matches("ref: ")
            .trim()
            .to_string();
        let head_commit =
            fs::read_to_string(self.dir.join(head_ref)).context("read commit of ref")?;
        fs::write(self.dir.join("refs/heads").join(branch), head_commit).context("write branch")?;

        Ok(())
    }

    /// Switches to the branch with the given name.
    pub fn switch_branch(&self, branch: &str) -> Result<()> {
        if !self.branch_exists(branch)? {
            Err(Error::BranchNotFound(branch.to_string()))?;
        }
        // Update HEAD to reference the new branch
        let branch_ref = format!("ref: refs/heads/{}\n", branch);
        fs::write(self.dir.join("HEAD"), branch_ref).context("write HEAD")?;

        Ok(())
    }

    /// Checks if a branch with the given name exists.
    pub fn branch_exists(&self, branch: &str) -> Result<bool> {
        Ok(fs::read_dir(self.dir.join("refs/heads"))?
            .filter_map(Result::ok)
            .any(|entry| entry.file_name() == branch))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_initialize_repository() {
        let temp_dir = tempdir().unwrap();
        let temp_dir_path = temp_dir.path().to_path_buf();

        Repository::init(&temp_dir_path).unwrap();

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
