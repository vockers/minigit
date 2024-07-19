use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
    #[error("Branch '{0}' not found")]
    BranchNotFound(String),
    #[error("A branch named '{0}' already exists")]
    BranchAlreadyExists(String),
}
