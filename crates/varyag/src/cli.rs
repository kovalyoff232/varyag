use clap::{Parser, Subcommand};
use crate::commands::{send::SendCommand, listen::ListenCommand, bridge::BridgeCommand};
use clap_complete::Shell;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Send a network request")]
    Send(SendCommand),
    #[command(about = "Listen for incoming traffic")]
    Listen(ListenCommand),
    #[command(about = "Create a tunnel to a local port")]
    Bridge(BridgeCommand),
    #[command(about = "Generate shell completions")]
    GenerateCompletion {
        #[arg(value_enum)]
        shell: Shell,
    },
}