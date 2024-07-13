use std::{env, time::SystemTime};

use anyhow::{Context, Result};

use super::{Kind, Object};

pub fn write_commit(tree_hash: &str, parent_hash: Option<&str>, message: &str) -> Result<String> {
    use std::fmt::Write;
    let mut commit = String::new();
    writeln!(commit, "tree {}", tree_hash)?;
    if let Some(parent_hash) = parent_hash {
        writeln!(commit, "parent {}", parent_hash)?;
    }
    let (name, email) =
        if let (Some(name), Some(email)) = (env::var_os("NAME"), env::var_os("EMAIL")) {
            let name = name
                .into_string()
                .map_err(|_| anyhow::anyhow!("$NAME is invalid UTF-8"))?;
            let email = email
                .into_string()
                .map_err(|_| anyhow::anyhow!("$EMAIL is invalid UTF-8"))?;
            (name, email)
        } else {
            (
                String::from("Vincent Ockers"),
                String::from("vincentbockers@gmail.com"),
            )
        };
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    writeln!(commit, "author {name} <{email}> {} +0000", time.as_secs())?;
    writeln!(
        commit,
        "committer {name} <{email}> {} +0000",
        time.as_secs()
    )?;
    writeln!(commit, "")?;
    writeln!(commit, "{message}")?;
    Object {
        kind: Kind::Commit,
        size: commit.len() as u64,
        reader: commit.as_bytes(),
    }
    .write_to_objects()
    .context("write commit object")
}
