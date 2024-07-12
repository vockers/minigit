pub mod commands;
pub mod object;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{cat_file, hash_object, init};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Init { directory } => {
            init::initialize_repository(directory)?;
        }
        Commands::CatFile { object } => {
            // TODO: pretty print (-p)
            cat_file::print_object(&object)?;
        }
        Commands::HashObject { file, write } => {
            hash_object::hash_object(&file, write)?;
        }
    }

    Ok(())
}
