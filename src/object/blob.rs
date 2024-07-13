use std::{fs, io::Read, path::Path};

use anyhow::Result;

use super::{Object, ObjectType};

impl Object<()> {
    pub fn blob_from_file(file: &Path) -> Result<Object<impl Read>> {
        let f = fs::File::open(file)?;
        Ok(Object {
            kind: ObjectType::Blob,
            size: f.metadata()?.len(),
            reader: f,
        })
    }
}
