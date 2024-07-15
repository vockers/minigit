use std::path::PathBuf;

use clap::Subcommand;

pub mod branch;
pub mod cat_file;
pub mod commit;
pub mod commit_tree;
pub mod hash_object;
pub mod init;
pub mod ls_tree;
pub mod write_tree;

#[derive(Subcommand)]
pub enum Commands {
    /// Create an empty Git repository or reinitialize an existing one
    Init {
        directory: Option<PathBuf>,
    },
    /// Provide contents or details of repository objects
    CatFile {
        object: String,
    },
    /// Compute object ID and optionally create an object from a file
    HashObject {
        file: PathBuf,

        /// Write the object into the object database
        #[clap(short)]
        write: bool,
    },
    /// List the contents of a tree object
    LsTree {
        treeish: String,

        /// List only filenames
        #[clap(short, long)]
        name_only: bool,
    },
    /// Create a tree object from the current index
    WriteTree {},
    /// Create a new commit object
    CommitTree {
        tree_hash: String,
        #[clap(short)]
        parent_hash: Option<String>,
        #[clap(short)]
        message: String,
    },
    /// Record changes to the repository
    Commit {
        #[clap(short)]
        message: String,
    },
    Branch {},
}
