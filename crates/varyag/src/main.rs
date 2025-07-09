use clap::{Parser, CommandFactory};
use cli::Cli;
use commands::{send::handle_send, listen::handle_listen, bridge::handle_bridge};
use std::io;

mod cli;
mod commands;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Send(command) => {
            handle_send(command).await;
        }
        cli::Commands::Listen(command) => {
            handle_listen(command).await;
        }
        cli::Commands::Bridge(command) => {
            handle_bridge(command).await;
        }
        cli::Commands::GenerateCompletion { shell } => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            clap_complete::generate(shell, &mut cmd, name, &mut io::stdout());
        }
    }
}
