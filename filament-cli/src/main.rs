mod build;
mod new;
mod run;
mod storage;

use std::path::PathBuf;

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};

use crate::{build::BuildCommand, new::NewCommand};

#[derive(Parser)]
#[command(name = "filament")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    New(NewCommand),
    Build(BuildCommand),
    /// Run a Filament application from a manifest
    Run {
        /// Path to the program manifest (filament.toml)
        #[arg(value_name = "MANIFEST")]
        manifest: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Cli::parse();
    let Some(command) = args.command else {
        Cli::command().print_long_help()?;
        return Ok(());
    };

    match command {
        Commands::New(command) => command.invoke(),
        Commands::Build(command) => command.invoke(),
        Commands::Run { manifest } => Ok(run::run_manifest(manifest).await?),
    }
}
