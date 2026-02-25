//! FemtoClaw CLI Application.
//!
//! Command-line interface for FemtoClaw Industrial Agent Runtime.

pub mod commands;

use clap::Parser;
use tracing::Level;

#[derive(Parser, Debug)]
#[command(name = "femtoclaw")]
#[command(about = "FemtoClaw Industrial Agent Runtime", long_about = None)]
pub struct Args {
    #[arg(long, default_value = "echo")]
    pub brain: String,

    #[arg(long)]
    pub config: Option<String>,

    #[arg(long, default_value = "info")]
    pub log_level: String,

    #[command(subcommand)]
    pub command: Option<commands::Command>,
}

pub async fn run() -> anyhow::Result<()> {
    let args = Args::parse();

    let level: Level = args.log_level.parse()
        .unwrap_or_else(|_| Level::INFO);
    
    tracing_subscriber::fmt()
        .with_max_level(level)
        .init();

    match args.command {
        Some(cmd) => commands::execute(cmd).await?,
        None => {
            tracing::info!("Starting FemtoClaw runtime...");
        }
    }

    Ok(())
}
