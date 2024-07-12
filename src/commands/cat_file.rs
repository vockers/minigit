use anyhow::Result;

use crate::object::Object;

pub fn run(hash: &str) -> Result<()> {
    let mut object = Object::read(hash)?;

    std::io::copy(&mut object.reader, &mut std::io::stdout())?;

    Ok(())
}
