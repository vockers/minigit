use std::{
    ffi::CStr,
    io::{BufRead, Read},
};

use anyhow::{Context, Result};

use crate::object::{Kind, Object};

pub fn run(hash: &str, name_only: bool) -> Result<()> {
    let mut tree = Object::read(hash)?;
    match tree.kind {
        crate::object::Kind::Tree => {
            let mut buf = Vec::new();
            let mut hash_buf = [0u8; 20];
            loop {
                let n = tree
                    .reader
                    .read_until(0, &mut buf)
                    .context("read next object in tree")?;
                if n == 0 {
                    break;
                }
                tree.reader
                    .read_exact(&mut hash_buf)
                    .context("read tree entry hash")?;
                let header = CStr::from_bytes_with_nul(&buf)
                    .context("invalid tree entry")?
                    .to_str()
                    .context("invalid utf-8 header")?;
                let hash = hex::encode(hash_buf);
                let Some((mode, name)) = header.split_once(' ') else {
                    anyhow::bail!("invalid tree entry");
                };
                if name_only {
                    println!("{}", name);
                } else {
                    let mode = mode.parse::<u32>().context("invalid mode")?;
                    let kind = Kind::from_mode(mode)?;
                    println!("{mode:0>6} {kind} {hash}    {name}");
                }
                buf.clear();
            }
        }
        _ => anyhow::bail!("Not a tree object"),
    }
    Ok(())
}
