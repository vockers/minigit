pub mod blob;
pub mod commit;
pub mod tree;

pub use commit::write_commit;
pub use tree::write_tree;

use std::{
    ffi::CStr,
    fs,
    io::{prelude::*, BufReader},
};

use anyhow::{Context, Result};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};

use crate::error::Error;

#[derive(Debug, PartialEq)]
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
}

impl ObjectType {
    pub fn from_mode(mode: u32) -> Result<Self> {
        let mode = mode / 1000;
        match mode {
            100 => Ok(ObjectType::Blob),
            040 => Ok(ObjectType::Tree),
            120 => Ok(ObjectType::Blob),
            160 => Ok(ObjectType::Commit),
            _ => anyhow::bail!("Unknown mode: {}", mode),
        }
    }
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self {
            ObjectType::Blob => "blob",
            ObjectType::Tree => "tree",
            ObjectType::Commit => "commit",
        };
        write!(f, "{}", kind)
    }
}

pub struct Object<R> {
    pub kind: ObjectType,
    pub size: u64,
    pub reader: R,
}

impl Object<()> {
    // TODO: abbreviated hash
    pub fn read(hash: &str) -> Result<Object<impl BufRead>> {
        let object_path = format!(".git/objects/{}/{}", &hash[..2], &hash[2..]);
        let f = fs::File::open(object_path).map_err(|_| Error::ObjectNotFound(hash.to_string()))?;

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
            "blob" => ObjectType::Blob,
            "tree" => ObjectType::Tree,
            "commit" => ObjectType::Commit,
            _ => anyhow::bail!("Unknown object kind: {}", kind),
        };

        let size = size.parse::<u64>().context("Failed to parse size")?;

        let reader = z.take(size);

        Ok(Object { kind, size, reader })
    }
}

impl<R> Object<R>
where
    R: Read,
{
    pub fn write(mut self, writer: impl Write) -> Result<String> {
        let writer = ZlibEncoder::new(writer, Compression::default());
        let mut writer = HashWriter {
            writer,
            hasher: Sha1::new(),
        };
        write!(writer, "{} {}\0", self.kind, self.size)?;
        std::io::copy(&mut self.reader, &mut writer).context("stream file into blob")?;
        writer.writer.finish()?;
        let hash = writer.hasher.finalize();
        Ok(hex::encode(hash))
    }

    pub fn write_to_objects(self) -> Result<String> {
        let temp_file = fs::File::create(".git/objects/.temp")?;
        let hash = self.write(temp_file)?;
        fs::create_dir_all(format!(".git/objects/{}/", &hash[..2]))
            .context("create subdir of .git/objects")?;
        fs::rename(
            ".git/objects/.temp",
            format!(".git/objects/{}/{}", &hash[..2], &hash[2..]),
        )?;

        Ok(hash)
    }
}

struct HashWriter<T> {
    writer: T,
    hasher: Sha1,
}

impl<W> Write for HashWriter<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.hasher.update(buf);
        self.writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_commit() {
        let hash = "defb1bfe50aa14da7248cc420d2a59c97ec8356c";
        let object = Object::read(hash).unwrap();
        let mut reader = BufReader::new(object.reader);

        let mut line = String::new();
        reader.read_to_string(&mut line).unwrap();
        assert_eq!(
            line,
            "tree ecabbf6e6c59d8d3d222685a369bb611803d3ce8\n\
            author Vincent Ockers <vincentbockers@gmail.com> 1720703241 +0200\n\
            committer Vincent Ockers <vincentbockers@gmail.com> 1720703241 +0200\n\n\
            Implement init command\n"
        );

        assert_eq!(object.kind, ObjectType::Commit);
    }

    #[test]
    fn test_read_blob() {
        let hash = "ea8c4bf7f35f6f77f75d92ad8ce8349f6e81ddba";
        let object = Object::read(hash).unwrap();
        let mut reader = BufReader::new(object.reader);

        let mut line = String::new();
        reader.read_to_string(&mut line).unwrap();
        assert_eq!(line, "/target\n");

        assert_eq!(object.kind, ObjectType::Blob);
    }

    #[test]
    fn test_get_hash_of_file() {
        use std::path::Path;
        let path = Path::new(".git/objects/ea/8c4bf7f35f6f77f75d92ad8ce8349f6e81ddba");
        let object = Object::blob_from_file(&path).unwrap();
        let hash = Object::write(object, std::io::sink()).unwrap();
        assert_eq!(hash, "50421f06294fa5c8578c630ec50ae9be47279d58");
    }
}
