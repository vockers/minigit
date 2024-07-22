use std::{fs, os::unix::fs::PermissionsExt, path::Path};

use anyhow::Result;

use crate::repository::Repository;

use super::{Object, ObjectType};

/// Recursively write a tree object to the repository
pub fn write_tree(path: &Path, repo: &Repository) -> Result<String> {
    let mut entries = vec![];

    let dir = fs::read_dir(path)?;
    for entry in dir {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if name.starts_with(".") {
            continue;
        }

        let meta = entry.metadata()?;
        let mut mode = meta.permissions().mode();
        let hash = if meta.is_dir() {
            // Trees don't have bits for executable permissions
            mode = 0o40000;
            write_tree(&path, &repo)?
        } else {
            Object::blob_from_file(&path)?.write_to_objects(&repo)?
        };

        let hash = hex::decode(&hash)?;
        entries.push((mode, name, hash));
    }

    // Git stores entries in a tree in alphabetical order
    entries.sort_by(|(_, a, _), (_, b, _)| a.cmp(&b));
    // format: "<mode> <name>\0<hash>"
    let entries: Vec<u8> = entries
        .into_iter()
        .flat_map(|(mode, name, hash)| {
            let header = format!("{:o} {}\0", mode, name);
            [header.as_bytes(), &hash].concat()
        }) //"{:o} {}\0{}", mode, name, hash))
        .collect();

    let object = Object {
        kind: ObjectType::Tree,
        size: entries.len() as u64,
        reader: entries.as_slice(),
    };
    Ok(object.write_to_objects(&repo)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_write_tree() {
        use crate::commands::init;
        let temp_dir = tempdir().unwrap();
        let temp_dir_path = temp_dir.path().to_path_buf();
        init::run(Some(temp_dir_path.clone())).unwrap();

        let repo = Repository::from_path(&temp_dir_path).unwrap();

        let foo_dir = temp_dir_path.join("foo");
        fs::create_dir(&foo_dir).unwrap();
        fs::write(foo_dir.join("bar"), "Hello Test\n").unwrap();
        fs::write(temp_dir_path.join("hello.txt"), "Hello World\n").unwrap();
        let hash = write_tree(&temp_dir_path, &repo).unwrap();
        assert_eq!(hash, "817795ce05795f9aa7bc8b744d2c57b2cffcf15c");
    }
}
