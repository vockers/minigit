pub mod commands;
pub mod object;
pub mod repository;

use anyhow::Result;
use clap::Parser;
use commands::{cat_file, hash_object, init, ls_tree, write_tree, Commands};

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
    }

    Ok(())
}
