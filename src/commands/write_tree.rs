use std::path::Path;

use anyhow::Result;

use crate::object::Object;

pub fn run() -> Result<()> {
    let hash = Object::write_tree(Path::new("."))?;
    println!("{}", hash);
    Ok(())
}
