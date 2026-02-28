//! Interactive REPL for FemtoClaw.

use std::io::{self, Write};

use femtoclaw::{Agent, Config};

fn config_for_brain(brain: &str) -> Config {
    let mut config = Config::default();
    config.brain.backend = brain.to_string();
    config
}

pub async fn run_repl(brain: &str) -> anyhow::Result<()> {
    println!("FemtoClaw Industrial Agent Runtime");
    println!("Type /help for commands, /quit to exit.\n");

    let agent = Agent::new(config_for_brain(brain))?;
    let mut history: Vec<String> = Vec::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        match input {
            "/quit" | "/exit" => break,
            "/help" => {
                println!("Commands:");
                println!("  /help     - Show this help");
                println!("  /quit     - Exit the REPL");
                println!("  /clear    - Clear history");
                println!("  /history  - Show command history");
                println!("  /reset    - Reset agent memory");
                continue;
            }
            "/clear" => {
                history.clear();
                continue;
            }
            "/history" => {
                for (i, cmd) in history.iter().enumerate() {
                    println!("{}: {}", i + 1, cmd);
                }
                continue;
            }
            "/reset" => {
                agent.reset().await;
                println!("Agent memory reset.");
                continue;
            }
            _ => {}
        }

        history.push(input.to_string());

        match agent.run(input).await {
            Ok(response) => {
                println!("{}", response);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    println!("Goodbye!");
    Ok(())
}

pub async fn run_once(brain: &str, prompt: &str) -> anyhow::Result<()> {
    let agent = Agent::new(config_for_brain(brain))?;
    let response = agent.run(prompt).await?;
    println!("{}", response);
    Ok(())
}
