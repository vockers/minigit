use std::{env, fmt::Write, time::SystemTime};

use crate::repository::Repository;

use super::{
    error::{ObjectError, Result},
    Object, ObjectType,
};

/// Write a commit object to the repository.
pub fn write_commit(
    tree_hash: &str,
    parent_hash: Option<&str>,
    message: &str,
    repo: &Repository,
) -> Result<String> {
    let mut commit = String::new();

    // impl Write for String can't fail
    writeln!(commit, "tree {}", tree_hash).unwrap();

    // Write parent hash if it exists
    if let Some(parent_hash) = parent_hash {
        writeln!(commit, "parent {}", parent_hash).unwrap();
    }
    // Read author and committer from environment variables, or me as default :)
    let (name, email) = env::var("NAME")
        .ok()
        .zip(env::var("EMAIL").ok())
        .unwrap_or_else(|| {
            (
                String::from("Vincent Ockers"),
                String::from("vincentbockers@gmail.com"),
            )
        });
    // Read the current time
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|_| ObjectError::Other("failed to get time".to_string()))?
        .as_secs();

    writeln!(commit, "author {name} <{email}> {time} +0000").unwrap();
    writeln!(commit, "committer {name} <{email}> {time} +0000").unwrap();
    writeln!(commit).unwrap();
    writeln!(commit, "{message}").unwrap();

    Object {
        kind: ObjectType::Commit,
        size: commit.len() as u64,
        reader: commit.as_bytes(),
    }
    .write_to_objects(repo)
}
