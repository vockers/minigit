use std::{fs, path::Path};

use anyhow::Result;

pub fn init(directory: &Path) -> Result<()> {
    let git_dir = directory.join(".git");
    fs::create_dir(&git_dir)?;
    fs::create_dir(git_dir.join("objects"))?;
    fs::create_dir(git_dir.join("refs"))?;
    fs::write(git_dir.join("HEAD"), "ref: refs/heads/main\n")?;
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
