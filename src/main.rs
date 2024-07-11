mod commands;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::init;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init { directory: Option<PathBuf> }
}

fn main() -> Result<()> {
    let args = Cli::parse(); 

    match args.command {
        Commands::Init { directory } => {
            init::initialize_repository(directory)?;
        }
    }

    Ok(())
}

