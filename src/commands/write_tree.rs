use std::{fs, os::unix::fs::PermissionsExt, path::Path};

use anyhow::Result;

use crate::object::{Kind, Object};

// TODO: write tests
fn write_tree(path: &Path) -> Result<String> {
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

pub fn run() -> Result<()> {
    let hash = write_tree(Path::new("."))?;
    println!("{}", hash);
    Ok(())
}
