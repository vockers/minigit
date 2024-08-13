use std::{io, path::PathBuf};

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ObjectError>;

#[derive(Error, Debug)]
pub enum ObjectError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("could not open '{0}' for reading: {1}")]
    Open(PathBuf, io::Error),

    #[error("could not parse object: {0}")]
    Parse(String),

    #[error("unknown mode '{0}'")]
    UnknownMode(u32),

    #[error("unknown object type: {0}")]
    UnknownType(String),

    #[error("object with hash '{0}' not found")]
    NotFound(String),

    #[error("{0}")]
    Other(String),
}
