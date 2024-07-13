use std::{env, fmt::Write, time::SystemTime};

use anyhow::{Context, Result};

use super::{Object, ObjectType};

pub fn write_commit(tree_hash: &str, parent_hash: Option<&str>, message: &str) -> Result<String> {
    let mut commit = String::new();

    writeln!(commit, "tree {}", tree_hash)?;

    if let Some(parent_hash) = parent_hash {
        writeln!(commit, "parent {}", parent_hash)?;
    }
    let (name, email) = env::var("NAME")
        .ok()
        .zip(env::var("EMAIL").ok())
        .unwrap_or_else(|| {
            (
                String::from("Vincent Ockers"),
                String::from("vincentbockers@gmail.com"),
            )
        });
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    writeln!(commit, "author {name} <{email}> {time} +0000")?;
    writeln!(commit, "committer {name} <{email}> {time} +0000")?;
    writeln!(commit)?;
    writeln!(commit, "{message}")?;

    Object {
        kind: ObjectType::Commit,
        size: commit.len() as u64,
        reader: commit.as_bytes(),
    }
    .write_to_objects()
    .context("write commit object")
}
