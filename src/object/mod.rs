pub mod blob;
pub mod commit;
pub mod error;
pub mod tree;

pub use commit::write_commit;
pub use tree::write_tree;

use error::{ObjectError, Result};
use std::{
    ffi::CStr,
    fs,
    io::{prelude::*, BufReader},
};

use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};

use crate::repository::Repository;

#[derive(Debug, PartialEq)]
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
}

impl ObjectType {
    /// Returns the mode of the object type
    pub fn from_mode(mode: u32) -> Result<Self> {
        let mode = mode / 1000;
        match mode {
            100 => Ok(ObjectType::Blob),
            40 => Ok(ObjectType::Tree),
            120 => Ok(ObjectType::Blob),
            160 => Ok(ObjectType::Commit),
            _ => Err(ObjectError::UnknownMode(mode)),
        }
    }
}

impl TryFrom<&str> for ObjectType {
    type Error = ObjectError;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "blob" => Ok(ObjectType::Blob),
            "tree" => Ok(ObjectType::Tree),
            "commit" => Ok(ObjectType::Commit),
            _ => Err(ObjectError::Other(format!(
                "Unknown object type: {}",
                value
            ))),
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
    /// Returns an object from the objects directory of the repository (.git/objects)
    pub fn read(hash: &str, repo: &Repository) -> Result<Object<impl BufRead>> {
        let object_path = repo
            .get_path()
            .join("objects")
            .join(&hash[..2])
            .join(&hash[2..]);
        let f = fs::File::open(object_path).map_err(|_| ObjectError::NotFound(hash.to_string()))?;

        let z = ZlibDecoder::new(f);
        let mut z = BufReader::new(z);
        let mut buf = Vec::new();
        z.read_until(0, &mut buf)?;
        let header = CStr::from_bytes_with_nul(&buf)
            .map_err(|_| ObjectError::Parse(hash.to_string()))?
            .to_str()
            .map_err(|_| ObjectError::Parse(hash.to_string()))?;

        let (kind, size) = header
            .split_once(' ')
            .ok_or(ObjectError::Parse(hash.to_string()))?;

        let kind = ObjectType::try_from(kind)?;

        let size = size
            .parse::<u64>()
            .map_err(|_| ObjectError::Parse(hash.to_string()))?;

        let reader = z.take(size);

        Ok(Object { kind, size, reader })
    }
}

impl<R> Object<R>
where
    R: Read,
{
    /// Write the object to a writer, returning the hash of the object
    pub fn write(mut self, writer: impl Write) -> Result<String> {
        let writer = ZlibEncoder::new(writer, Compression::default());
        let mut writer = HashWriter {
            writer,
            hasher: Sha1::new(),
        };
        write!(writer, "{} {}\0", self.kind, self.size)?;
        std::io::copy(&mut self.reader, &mut writer)?;
        writer.writer.finish()?;
        let hash = writer.hasher.finalize();
        Ok(hex::encode(hash))
    }

    /// Write the object to the objects directory of the repository (.git/objects)
    pub fn write_to_objects(self, repo: &Repository) -> Result<String> {
        // Since hash is calculated during writing, we need to write to a temp file first
        let temp_file_path = repo.get_path().join("objects/.temp");
        let temp_file = fs::File::create(&temp_file_path)?;
        let hash = self.write(temp_file)?;
        let object_dir = repo.get_path().join("objects").join(&hash[..2]);
        fs::create_dir_all(&object_dir)?;
        fs::rename(temp_file_path, object_dir.join(&hash[2..]))?;

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
        let repo = Repository::from_path(".").unwrap();
        let hash = "defb1bfe50aa14da7248cc420d2a59c97ec8356c";
        let object = Object::read(hash, &repo).unwrap();
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
        let repo = Repository::from_path(".").unwrap();
        let hash = "ea8c4bf7f35f6f77f75d92ad8ce8349f6e81ddba";
        let object = Object::read(hash, &repo).unwrap();
        let mut reader = BufReader::new(object.reader);

        let mut line = String::new();
        reader.read_to_string(&mut line).unwrap();
        assert_eq!(line, "/target\n");

        assert_eq!(object.kind, ObjectType::Blob);
    }

    #[test]
    fn test_get_hash_of_file() {
        let path = ".git/objects/ea/8c4bf7f35f6f77f75d92ad8ce8349f6e81ddba";
        let object = Object::blob_from_file(&path).unwrap();
        let hash = Object::write(object, std::io::sink()).unwrap();
        assert_eq!(hash, "50421f06294fa5c8578c630ec50ae9be47279d58");
    }
}
