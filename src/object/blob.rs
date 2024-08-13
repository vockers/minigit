use std::{fs, io::Read, path::Path};

use super::{
    error::{ObjectError, Result},
    Object, ObjectType,
};

impl Object<()> {
    /// Returns a new blob object from a file.
    pub fn blob_from_file<P: AsRef<Path>>(file: P) -> Result<Object<impl Read>> {
        let file = file.as_ref();
        let f = fs::File::open(file).map_err(|e| ObjectError::Open(file.to_owned(), e))?;
        Ok(Object {
            kind: ObjectType::Blob,
            size: f
                .metadata()
                .map_err(|e| ObjectError::Open(file.to_owned(), e))?
                .len(),
            reader: f,
        })
    }
}
