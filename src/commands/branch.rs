use std::{fs, path::Path};

use anyhow::{Context, Result};
use colored::*;

fn collect_entries<P: AsRef<Path>>(path: P, trim_path: &str) -> Result<Vec<String>> {
    let mut entries = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            // Recursively collect entries from subdirectories.
            entries.extend(
                collect_entries(&entry_path, trim_path)
                    .context("collect branches in subdirectories")?,
            );
        } else {
            entries.push(
                entry_path
                    .to_string_lossy()
                    .trim_start_matches(trim_path)
                    .to_string(),
            );
        }
    }

    Ok(entries)
}

// TODO: write tests
/// List branches.
pub fn run(all: bool) -> Result<()> {
    let heads_path = Path::new(".git/refs/heads");
    let mut heads_entries: Vec<String> =
        collect_entries(heads_path, ".git/refs/heads/").context("collect branches")?;
    heads_entries.sort();

    let current_head = fs::read_to_string(".git/HEAD")
        .context("read HEAD")?
        .trim_start_matches("ref: refs/heads/")
        .trim()
        .to_string();

    // Print branches, '*' indicates the current branch.
    for entry in heads_entries {
        if current_head == entry {
            println!("* {}", entry.green());
        } else {
            println!("  {}", entry);
        }
    }

    // Print remote branches if `--all` is passed.
    let remotes_path = Path::new(".git/refs/remotes");
    if all && remotes_path.exists() {
        let mut remotes_entries: Vec<String> =
            collect_entries(remotes_path, ".git/refs/").context("collect remote branches")?;
        remotes_entries.sort();

        for entry in remotes_entries {
            println!("  {}", entry.red());
        }
    }

    Ok(())
}
