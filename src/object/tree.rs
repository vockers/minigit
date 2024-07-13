use std::{fs, os::unix::fs::PermissionsExt, path::Path};

use anyhow::Result;

use super::{Kind, Object};

pub fn write_tree(path: &Path) -> Result<String> {
    let mut entries = vec![];
    let mut dir = fs::read_dir(path)?;
    while let Some(entry) = dir.next() {
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
            write_tree(&path)?
        } else {
            Object::blob_from_file(&path)?.write_to_objects()?
        };
        let hash = hex::decode(&hash)?;
        entries.push((mode, name, hash));
    }
    entries.sort_by(|(_, a, _), (_, b, _)| a.cmp(&b));
    let entries: Vec<Vec<u8>> = entries
        .into_iter()
        .map(|(mode, name, hash)| {
            let header = format!("{:o} {}\0", mode, name);
            [header.as_bytes(), &hash].concat()
        }) //"{:o} {}\0{}", mode, name, hash))
        .collect();
    let entries = entries.concat();
    let object = Object {
        kind: Kind::Tree,
        size: entries.len() as u64,
        reader: entries.as_slice(),
    };
    Ok(object.write_to_objects()?)
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
        let foo_dir = temp_dir_path.join("foo");
        fs::create_dir(&foo_dir).unwrap();
        fs::write(foo_dir.join("bar"), "Hello Test\n").unwrap();
        fs::write(temp_dir_path.join("hello.txt"), "Hello World\n").unwrap();
        let hash = write_tree(&temp_dir_path).unwrap();
        assert_eq!(hash, "817795ce05795f9aa7bc8b744d2c57b2cffcf15c");
    }
}
