//! Interactive REPL for FemtoClaw.

use std::io::{self, Write};

pub async fn run_repl(brain: &str) -> anyhow::Result<()> {
    println!("FemtoClaw Industrial Agent Runtime");
    println!("Brain: {}", brain);
    println!("Type /help for commands, /quit to exit.\n");

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
                println!("  /help   - Show this help");
                println!("  /quit   - Exit the REPL");
                println!("  /clear  - Clear history");
                println!("  /history - Show command history");
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
            _ => {}
        }

        history.push(input.to_string());

        let response = process_input(input, brain).await;
        println!("{}", response);
    }

    println!("Goodbye!");
    Ok(())
}

async fn process_input(input: &str, _brain: &str) -> String {
    if input.starts_with('/') {
        return format!("Unknown command: {}", input);
    }

    format!("Processing: {}...", input)
}
