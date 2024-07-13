use std::path::Path;

use anyhow::Result;

use crate::object;

pub fn run() -> Result<()> {
    let hash = object::write_tree(Path::new("."))?;

    println!("{}", hash);

    Ok(())
}
