use clap::Args;
use std::path::PathBuf;
use url::Url;
use vyg_core::{http_client, net_client, ws_client};
use vyg_display::{json::pretty_print_json, table::print_key_value_table, logger};


#[derive(Args, Debug)]
#[command(arg_required_else_help = true)]
pub struct SendCommand {
    /// The target URL or address (e.g., `https://api.example.com`, `example.com`, `ws://host:port`, `tcp://host:port`).
    /// If no scheme is provided, `http://` is assumed.
    #[arg()]
    pub destination: String,

    /// Optional: The HTTP method (e.g., GET, POST).
    /// If body items or --data-file are present, it defaults to POST. Otherwise, it defaults to GET.
    #[arg(value_parser = ["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"])]
    pub method: Option<String>,

    /// Request body items for HTTP requests (e.g., `name=value` `field:=json_value`).
    #[arg(num_args = 0..)]
    pub body: Vec<String>,

    /// Custom headers for HTTP requests (e.g., `-H "X-API-Key: secret"`).
    #[arg(short = 'H', long = "header")]
    pub headers: Vec<String>,

    /// Send request body from a file.
    #[arg(long, value_name = "FILE_PATH")]
    pub data_file: Option<PathBuf>,

    /// Use interactive mode for TCP connections.
    #[arg(short, long)]
    pub interactive: bool,

    /// Disable proxy for this request.
    #[arg(long)]
    pub noproxy: bool,
}

pub async fn handle_send(command: SendCommand) {
    let destination = if !command.destination.contains("://") {
        format!("http://{}", command.destination)
    } else {
        command.destination.clone()
    };

    if let Ok(url) = Url::parse(&destination) {
        match url.scheme() {
            "http" | "https" => handle_http_request(command, url).await,
            "ws" | "wss" => handle_ws_request(command, url).await,
            "tcp" => handle_tcp_request(command, &destination).await,
            "udp" => handle_udp_request(command, &destination).await,
            _ => logger::error(&format!("Unsupported protocol: {}", url.scheme())),
        }
    } else if let Some(addr) = destination.strip_prefix("tcp://").or_else(|| destination.strip_prefix("udp://")) {
        if destination.starts_with("tcp://") {
            handle_tcp_request(command, addr).await;
        } else {
            handle_udp_request(command, addr).await;
        }
    }
    else {
        logger::error(&format!("Invalid URL or address: {}", command.destination));
    }
}

async fn handle_http_request(command: SendCommand, url: Url) {
    logger::info(&format!("Sending HTTP request to: {}", url));
    let method = command.method.unwrap_or_else(|| {
        if command.data_file.is_some() || !command.body.is_empty() {
            "POST".to_string()
        } else {
            "GET".to_string()
        }
    });

    let http_request = http_client::HttpRequest {
        url: url.to_string(),
        method,
        headers: command.headers,
        body: command.body,
        data_file: command.data_file,
        noproxy: command.noproxy,
    };

    match http_client::send_request(http_request).await {
        Ok(response) => {
            logger::info(&format!("Status: {}", response.status));
            
            let headers_for_table: Vec<(String, String)> = response.headers.iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect();
            if !headers_for_table.is_empty() {
                println!("Headers:");
                if let Err(e) = print_key_value_table(&headers_for_table) {
                    logger::error(&format!("Failed to print headers table: {}", e));
                }
            }
            
            println!("\nBody:");
            pretty_print_json(&response.body);
        },
        Err(e) => logger::error(&format!("Request failed: {}", e)),
    }
}

async fn handle_ws_request(command: SendCommand, url: Url) {
    let message = if command.body.is_empty() {
        None
    } else {
        Some(command.body.join(" "))
    };
    if let Err(e) = ws_client::connect_ws(url.as_str(), message).await {
        logger::error(&format!("WebSocket connection failed: {}", e));
    }
}

async fn handle_tcp_request(command: SendCommand, address: &str) {
    let data = command.body.join(" ").into_bytes();
    if let Err(e) = net_client::send_tcp_request(address, data, command.interactive).await {
        logger::error(&format!("TCP request failed: {}", e));
    }
}

async fn handle_udp_request(command: SendCommand, address: &str) {
    let data = command.body.join(" ").into_bytes();
    if let Err(e) = net_client::send_udp_request(address, data).await {
        logger::error(&format!("UDP request failed: {}", e));
    }
}
