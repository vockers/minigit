use anyhow::Result;

use crate::object;

pub fn run(tree_hash: &str, parent_hash: Option<&str>, message: &str) -> Result<()> {
    let hash = object::write_commit(tree_hash, parent_hash, message)?;
    println!("{}", hash);
    Ok(())
}
