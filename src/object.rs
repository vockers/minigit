use std::{
    ffi::CStr,
    fs,
    io::{prelude::*, BufReader},
};

use anyhow::{Context, Result};
use flate2::read::ZlibDecoder;

pub enum Kind {
    Blob,
    Tree,
    Commit,
}

pub struct Object<R> {
    pub kind: Kind,
    pub reader: R,
}

impl Object<()> {
    // TODO: abbreviated hash
    pub fn read(hash: &str) -> Result<Object<impl Read>> {
        let f = fs::File::open(format!(".git/objects/{}/{}", &hash[..2], &hash[2..]))?;

        let z = ZlibDecoder::new(f);
        let mut z = BufReader::new(z);
        let mut buf = Vec::new();
        z.read_until(0, &mut buf)
            .context("read header from .git/objects file")?;
        let header = CStr::from_bytes_with_nul(&buf)
            .context("Failed to read header")?
            .to_str()
            .context(".git/objects file header isn't valid UTF-8")?;

        let (kind, size) = header.split_once(' ').context("Failed to parse header")?;

        let kind = match kind {
            "blob" => Kind::Blob,
            "tree" => Kind::Tree,
            "commit" => Kind::Commit,
            _ => anyhow::bail!("Unknown object kind: {}", kind),
        };

        let size = size.parse::<u64>().context("Failed to parse size")?;

        let reader = z.take(size);

        Ok(Object { kind, reader })
    }
}
