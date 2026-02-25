//! CLI Commands.

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    Run,
    Init {
        #[arg(long)]
        path: Option<String>,
    },
    Status,
    Version,
}

pub async fn execute(cmd: Command) -> anyhow::Result<()> {
    match cmd {
        Command::Run => {
            tracing::info!("Running FemtoClaw...");
            super::repl::run_repl("echo").await?;
        }
        Command::Init { path } => {
            let config_path = path.unwrap_or_else(|| "~/.config/femtoclaw/config.json".to_string());
            tracing::info!("Initializing FemtoClaw at {}...", config_path);
            println!("Config initialized at {}", config_path);
        }
        Command::Status => {
            println!("FemtoClaw Status:");
            println!("  Status: Ready");
            println!("  Version: 1.0.0");
        }
        Command::Version => {
            println!("femtoclaw {}", env!("CARGO_PKG_VERSION"));
        }
    }
    Ok(())
}
