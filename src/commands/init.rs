use std::{env, path::PathBuf};

use anyhow::Result;

use crate::repository;

pub fn run(directory: Option<PathBuf>) -> Result<()> {
    let directory = if let Some(directory) = directory {
        env::current_dir()?.join(directory)
    } else {
        env::current_dir()?
    };
    repository::init(&directory)?;
    println!(
        "Initialized empty Git repository in {}/.git",
        directory.display()
    );
    Ok(())
}
