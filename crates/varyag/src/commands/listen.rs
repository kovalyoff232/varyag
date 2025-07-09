use clap::{Args, Subcommand};
use vyg_core::net_listener;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct ListenCommand {
    #[command(subcommand)]
    pub protocol: Protocol,
}

#[derive(Subcommand, Debug)]
pub enum Protocol {
    #[command(about = "Listen for HTTP traffic")]
    Http(HttpArgs),
    #[command(about = "Listen for TCP traffic")]
    Tcp(TcpArgs),
    #[command(about = "Listen for WebSocket traffic")]
    Ws(WsArgs),
}

#[derive(Args, Debug)]
pub struct HttpArgs {
    #[arg(default_value = "8080")]
    pub port: u16,
    #[arg(long, value_name = "PATH", conflicts_with = "proxy_pass")]
    pub serve: Option<PathBuf>,
    #[arg(long, value_name = "URL", conflicts_with = "serve")]
    pub proxy_pass: Option<String>,
}

#[derive(Args, Debug)]
pub struct TcpArgs {
    pub port: u16,
    #[arg(long)]
    pub echo: bool,
}

#[derive(Args, Debug)]
pub struct WsArgs {
    pub port: u16,
    #[arg(long)]
    pub echo: bool,
}

pub async fn handle_listen(command: ListenCommand) {
    match command.protocol {
        Protocol::Http(args) => {
            if let Err(e) = net_listener::start_http_listener(args.port, args.serve, args.proxy_pass).await {
                eprintln!("Error: {}", e);
            }
        }
        Protocol::Tcp(args) => {
            if let Err(e) = net_listener::start_tcp_listener(args.port, args.echo).await {
                eprintln!("Error: {}", e);
            }
        }
        Protocol::Ws(args) => {
            if let Err(e) = net_listener::start_ws_listener(args.port, args.echo).await {
                eprintln!("Error: {}", e);
            }
        }
    }
}