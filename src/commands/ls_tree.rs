use std::{
    ffi::CStr,
    io::{BufRead, Read},
};

use anyhow::{bail, Context, Result};

use crate::{
    object::{Object, ObjectType},
    repository::Repository,
};

/// List the contents of a tree object.
pub fn run(hash: &str, name_only: bool) -> Result<()> {
    let repo = Repository::from_path(".")?;
    let mut tree = Object::read(hash, &repo)?;
    if tree.kind != ObjectType::Tree {
        bail!("Not a tree object");
    }

    let mut buf = Vec::new();
    let mut hash_buf = [0u8; 20];
    loop {
        // format: "<mode> <name>\0<hash>"
        // Read header until null byte
        let n = tree
            .reader
            .read_until(0, &mut buf)
            .context("read next object in tree")?;
        if n == 0 {
            break;
        }

        // Extract the header from the buffer
        let header = CStr::from_bytes_with_nul(&buf)
            .context("invalid tree entry")?
            .to_str()
            .context("invalid utf-8 header")?;
        let (mode, name) = header.split_once(' ').context("invalid tree entry")?;

        // Read the hash and convert it to a hex string
        tree.reader
            .read_exact(&mut hash_buf)
            .context("read tree entry hash")?;
        let hash = hex::encode(hash_buf);

        if name_only {
            println!("{}", name);
        } else {
            let mode = mode.parse::<u32>().context("invalid mode")?;
            let kind = ObjectType::from_mode(mode)?;
            println!("{mode:0>6} {kind} {hash}    {name}");
        }

        buf.clear();
    }

    Ok(())
}
