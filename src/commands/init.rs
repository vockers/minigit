use std::{env, fs, path::PathBuf};

use anyhow::Result;

pub fn initialize_repository(directory: Option<PathBuf>) -> Result<()> {
    let directory = if let Some(directory) = directory {
        env::current_dir()?.join(directory)
    } else {
        env::current_dir()?
    };
    let git_dir = directory.join(".git");
    fs::create_dir(&git_dir)?;
    fs::create_dir(git_dir.join("objects"))?;
    fs::create_dir(git_dir.join("refs"))?;
    fs::write(git_dir.join("HEAD"), "ref: refs/heads/main\n")?;
    println!("Initialized empty Git repository in {}", git_dir.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_initialize_repository() {
        let temp_dir = tempdir().unwrap();
        let temp_dir_path = temp_dir.path().to_path_buf();

        initialize_repository(Some(temp_dir_path.clone())).unwrap();

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
