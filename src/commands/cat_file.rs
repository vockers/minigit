use anyhow::Result;

use crate::object::Object;

pub fn print_object(hash: &str) -> Result<()> {
    let mut object = Object::read(hash)?;

    std::io::copy(&mut object.reader, &mut std::io::stdout())?;

    Ok(())
}
