use std::{fs, io::Read, path::Path};

use anyhow::Result;

use super::{Object, ObjectType};

impl Object<()> {
    /// Returns a new blob object from a file.
    pub fn blob_from_file<P: AsRef<Path>>(file: P) -> Result<Object<impl Read>> {
        let f = fs::File::open(file)?;
        Ok(Object {
            kind: ObjectType::Blob,
            size: f.metadata()?.len(),
            reader: f,
        })
    }
}
