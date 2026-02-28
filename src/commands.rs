//! CLI Commands.

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    Run {
        #[arg(long)]
        prompt: Option<String>,
    },
    Init {
        #[arg(long)]
        path: Option<String>,
    },
    Status,
    Version,
}

pub async fn execute(cmd: Command, brain: &str) -> anyhow::Result<()> {
    match cmd {
        Command::Run { prompt } => {
            tracing::info!(brain, "Running FemtoClaw");
            if let Some(prompt) = prompt {
                super::repl::run_once(brain, &prompt).await?;
            } else {
                super::repl::run_repl(brain).await?;
            }
        }
        Command::Init { path } => {
            let config_path = path.unwrap_or_else(|| "~/.config/femtoclaw/config.json".to_string());
            tracing::info!("Initializing FemtoClaw at {}...", config_path);
            println!("Config initialized at {}", config_path);
        }
        Command::Status => {
            println!("FemtoClaw Status:");
            println!("  Status: Ready");
            println!("  Version: {}", env!("CARGO_PKG_VERSION"));
            println!("  Brain: {}", brain);
        }
        Command::Version => {
            println!("femtoclaw {}", env!("CARGO_PKG_VERSION"));
        }
    }
    Ok(())
}
