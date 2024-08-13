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
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let git_dir = path.as_ref().join(".git");
        if !git_dir.exists() {
            Err(Error::NotGitRepository)?;
        }

        Ok(Self { dir: git_dir })
    }

    /// Initializes a new Git repository at the given directory.
    pub fn init(directory: &Path) -> Result<Repository> {
        let dir = directory.join(".git");
        if dir.exists() {
            Err(Error::AlreadyInitialized)?;
        }
        fs::create_dir_all(dir.join("objects"))?;
        fs::create_dir_all(dir.join("refs"))?;
        fs::write(dir.join("HEAD"), "ref: refs/heads/main\n")?;

        Ok(Repository { dir })
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
        Ok(fs::read_dir(self.dir.join("refs/heads"))
            .context("read branches")?
            .filter_map(Result::ok)
            .any(|entry| entry.file_name() == branch))
    }

    /// Returns the hash of the commit referenced by the given ref path.
    pub fn get_ref(&self, ref_path: &str) -> Result<String> {
        fs::read_to_string(self.dir.join(ref_path)).context("read ref")
    }

    /// Sets the hash of the commit referenced by the given ref path.
    pub fn set_ref(&self, ref_path: &str, hash: &str) -> Result<()> {
        fs::write(self.dir.join(ref_path), hash).context("write ref")
    }

    /// Returns the ref path of the current HEAD.
    pub fn get_head(&self) -> Result<String> {
        let head_ref = fs::read_to_string(self.dir.join("HEAD"))
            .context("read HEAD")?
            .trim_start_matches("ref: ")
            .trim()
            .to_string();
        Ok(head_ref)
    }

    /// Returns the root directory of the repository.
    pub fn get_root(&self) -> &Path {
        self.dir.parent().unwrap_or(Path::new("."))
    }

    /// Returns the path of the repository (`.git` directory)
    pub fn get_path(&self) -> &Path {
        self.dir.as_path()
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
        let repo = Repository::init(&temp_dir_path).unwrap();

        let git_dir = repo.dir;
        assert!(git_dir.exists());
        assert!(git_dir.join("objects").exists());
        assert!(git_dir.join("refs").exists());
        assert!(git_dir.join("HEAD").exists());
        assert_eq!(
            fs::read_to_string(git_dir.join("HEAD")).unwrap(),
            "ref: refs/heads/main\n"
        );
    }

    #[test]
    fn test_create_branch() {
        //let temp_dir = tempdir().unwrap();
        //let temp_dir_path = temp_dir.path().to_path_buf();
        //let repo = Repository::init(&temp_dir_path).unwrap();
        //
        //repo.create_branch("test").unwrap();
        //assert!(repo.branch_exists("test").unwrap());
    }
}
