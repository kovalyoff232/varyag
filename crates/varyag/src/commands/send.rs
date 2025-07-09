use clap::{Args, Subcommand};
use vyg_core::{http_client, net_client, ws_client};
use vyg_display::json::pretty_print_json;

#[derive(Args, Debug)]
pub struct SendCommand {
    #[command(subcommand)]
    pub protocol: Protocol,
}

#[derive(Subcommand, Debug)]
pub enum Protocol {
    #[command(about = "Send an HTTP(S) request")]
    Http(HttpArgs),
    #[command(about = "Send data over TCP")]
    Tcp(TcpArgs),
    #[command(about = "Connect to a WebSocket")]
    Ws(WsArgs),
}

#[derive(Args, Debug)]
pub struct HttpArgs {
    #[arg(default_value = "GET")]
    pub method: String,
    pub url: String,
    #[arg(num_args = 0..)]
    pub body: Vec<String>,
}

#[derive(Args, Debug)]
pub struct TcpArgs {
    pub address: String,
    #[arg(num_args = 0..)]
    pub data: Vec<String>,
}

#[derive(Args, Debug)]
pub struct WsArgs {
    pub url: String,
    #[arg(num_args = 0..)]
    pub message: Vec<String>,
}

pub async fn handle_send(command: SendCommand) {
    match command.protocol {
        Protocol::Http(args) => {
            let method = args.method.to_uppercase();
            match method.as_str() {
                "GET" => {
                    if !args.body.is_empty() {
                        eprintln!("Warning: Request body is not supported for GET requests and will be ignored.");
                    }
                    match http_client::send_get_request(&args.url).await {
                        Ok(response_body) => pretty_print_json(&response_body),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                "POST" => {
                    match http_client::send_post_request(&args.url, args.body).await {
                        Ok(response_body) => pretty_print_json(&response_body),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                _ => eprintln!("Unsupported HTTP method: {}", method),
            }
        }
        Protocol::Tcp(args) => {
            let data = args.data.join(" ").into_bytes();
            if let Err(e) = net_client::send_tcp_request(&args.address, data).await {
                eprintln!("Error: {}", e);
            }
        }
        Protocol::Ws(args) => {
            let message = if args.message.is_empty() {
                None
            } else {
                Some(args.message.join(" "))
            };
            if let Err(e) = ws_client::connect_ws(&args.url, message).await {
                eprintln!("Error: {}", e);
            }
        }
    }
}