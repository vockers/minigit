pub mod commands;
pub mod object;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{cat_file, init};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init { directory: Option<PathBuf> },
    CatFile { object: String },
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
    }

    Ok(())
}
