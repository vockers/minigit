pub mod commands;
pub mod object;
pub mod repository;

use anyhow::Result;
use clap::Parser;
use commands::Commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    commands::run(args.command)?;

    Ok(())
}
