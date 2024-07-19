use std::path::PathBuf;

use anyhow::Result;
use clap::Subcommand;

pub mod branch;
pub mod cat_file;
pub mod checkout;
pub mod commit;
pub mod commit_tree;
pub mod hash_object;
pub mod init;
pub mod ls_tree;
pub mod write_tree;

#[derive(Subcommand)]
pub enum Commands {
    /// Create an empty Git repository or reinitialize an existing one
    Init { directory: Option<PathBuf> },
    /// Provide contents or details of repository objects
    CatFile { object: String },
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
    /// List branches
    Branch {
        /// list both remote-tracking and local branche
        #[clap(short)]
        all: bool,
    },
    /// Switch branches
    Checkout {
        /// create and checkout a new branch
        #[clap(short = 'b')]
        create_branch: bool,
        branch: String,
    },
}

pub fn run(command: Commands) -> Result<()> {
    match command {
        Commands::Init { directory } => {
            init::run(directory)?;
        }
        Commands::CatFile { object } => {
            // TODO: pretty print (-p)
            cat_file::run(&object)?;
        }
        Commands::HashObject { file, write } => {
            hash_object::run(&file, write)?;
        }
        Commands::LsTree { treeish, name_only } => {
            ls_tree::run(&treeish, name_only)?;
        }
        Commands::WriteTree {} => {
            write_tree::run()?;
        }
        Commands::CommitTree {
            tree_hash,
            parent_hash,
            message,
        } => {
            commit_tree::run(&tree_hash, parent_hash.as_deref(), &message)?;
        }
        Commands::Commit { message } => {
            commit::run(&message)?;
        }
        Commands::Branch { all } => {
            branch::run(all)?;
        }
        Commands::Checkout {
            create_branch,
            branch,
        } => {
            checkout::run(create_branch, &branch)?;
        }
    }

    Ok(())
}
