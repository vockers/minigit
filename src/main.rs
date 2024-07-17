pub mod commands;
pub mod object;
pub mod repository;

use anyhow::Result;
use clap::Parser;
use commands::{
    branch, cat_file, checkout, commit, commit_tree, hash_object, init, ls_tree, write_tree,
    Commands,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
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
