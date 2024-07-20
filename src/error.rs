use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("branch '{0}' not found")]
    BranchNotFound(String),
    #[error("a branch named '{0}' already exists")]
    BranchAlreadyExists(String),
    #[error("object with hash '{0}' not found")]
    ObjectNotFound(String),
    #[error("not a git repository")]
    NotGitRepository,
}
