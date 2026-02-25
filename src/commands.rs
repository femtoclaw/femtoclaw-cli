//! CLI Commands.

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    Run,
    Init,
    Status,
}

pub async fn execute(cmd: Command) -> anyhow::Result<()> {
    match cmd {
        Command::Run => {
            tracing::info!("Running FemtoClaw...");
        }
        Command::Init => {
            tracing::info!("Initializing FemtoClaw...");
        }
        Command::Status => {
            tracing::info!("Checking FemtoClaw status...");
        }
    }
    Ok(())
}
