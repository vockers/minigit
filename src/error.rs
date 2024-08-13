use std::io;

use thiserror::Error;

use crate::object::error::ObjectError;

pub type Result<T> = std::result::Result<T, GitError>;

#[derive(Error, Debug)]
pub enum GitError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("{0}")]
    Object(#[from] ObjectError),

    #[error("branch '{0}' not found")]
    BranchNotFound(String),

    #[error("a branch named '{0}' already exists")]
    BranchAlreadyExists(String),

    #[error("not a git repository")]
    NotGitRepository,

    #[error("repository already initialized")]
    AlreadyInitialized,
}
