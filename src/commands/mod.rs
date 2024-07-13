use std::path::PathBuf;

use clap::Subcommand;

pub mod cat_file;
pub mod commit_tree;
pub mod hash_object;
pub mod init;
pub mod ls_tree;
pub mod write_tree;

#[derive(Subcommand)]
pub enum Commands {
    Init {
        directory: Option<PathBuf>,
    },
    CatFile {
        object: String,
    },
    HashObject {
        file: PathBuf,

        /// Write the object into the object database
        #[clap(short)]
        write: bool,
    },
    LsTree {
        treeish: String,

        /// List only filenames
        #[clap(short, long)]
        name_only: bool,
    },
    WriteTree {},
    CommitTree {
        tree_hash: String,
        #[clap(short)]
        parent_hash: Option<String>,
        #[clap(short)]
        message: String,
    },
}
