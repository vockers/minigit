use std::fs;

use anyhow::Result;
use colored::*;

pub fn run() -> Result<()> {
    let heads_dir = fs::read_dir(".git/refs/heads")?;
    let mut entries: Vec<String> = heads_dir
        .filter_map(|entry| {
            entry
                .ok()
                .map(|entry| entry.file_name().to_string_lossy().to_string())
        })
        .collect();

    entries.sort();

    let current_head = fs::read_to_string(".git/HEAD")?
        .trim_start_matches("ref: refs/heads/")
        .trim()
        .to_string();

    for entry in entries {
        if current_head == entry {
            println!("* {}", entry.green());
        } else {
            println!("  {}", entry);
        }
    }

    Ok(())
}
