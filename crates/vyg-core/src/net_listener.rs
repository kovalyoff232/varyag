use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use anyhow::Result;
use axum::{
    routing::{any, get_service},
    Router,
    http::{Request, StatusCode},
    body::Body,
    response::IntoResponse,
};
use std::net::SocketAddr;
use http_body_util::BodyExt;
use tokio::net::TcpListener;
use futures_util::{StreamExt, SinkExt};
use std::path::PathBuf;
use tower_http::services::ServeDir;

pub async fn start_tcp_listener(port: u16, echo: bool) -> Result<()> {
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    println!("Listening for TCP on port {}", port);

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Accepted connection from {}", addr);

        if echo {
            tokio::spawn(async move {
                let (mut reader, mut writer) = socket.split();
                if let Err(e) = io::copy(&mut reader, &mut writer).await {
                    eprintln!("Failed to copy data: {}", e);
                }
            });
        } else {
            tokio::spawn(async move {
                let mut buffer = vec![0; 1024];
                loop {
                    match socket.read(&mut buffer).await {
                        Ok(0) => {
                            println!("Connection closed by {}", addr);
                            return;
                        }
                        Ok(n) => {
                            let data = &buffer[..n];
                            if let Err(e) = io::stdout().write_all(data).await {
                                eprintln!("Failed to write to stdout: {}", e);
                                return;
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to read from socket: {}", e);
                            return;
                        }
                    }
                }
            });
        }
    }
}

pub async fn start_http_listener(port: u16, serve_path: Option<PathBuf>) -> Result<()> {
    let app = if let Some(path) = serve_path {
        println!("Serving files from {:?}", path);
        Router::new().nest_service("/", get_service(ServeDir::new(path)))
    } else {
        Router::new().route("/*path", any(inspect_request))
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Listening for HTTP on port {}", port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn inspect_request(req: Request<Body>) -> impl IntoResponse {
    println!("Request: {} {}", req.method(), req.uri());
    println!("Headers: {:#?}", req.headers());
    let body_bytes = req.into_body().collect().await.unwrap().to_bytes();
    if let Ok(body_str) = String::from_utf8(body_bytes.to_vec()) {
        if !body_str.is_empty() {
            println!("Body: {}", body_str);
        }
    } else {
        println!("Body: (non-UTF8 data)");
    }
    println!("---");

    (StatusCode::OK, "Request logged to console")
}

pub async fn start_ws_listener(port: u16, echo: bool) -> Result<()> {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    println!("Listening for WebSocket on port {}", port);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, echo));
    }

    Ok(())
}

async fn accept_connection(stream: tokio::net::TcpStream, echo: bool) {
    if let Ok(ws_stream) = tokio_tungstenite::accept_async(stream).await {
        let (mut write, mut read) = ws_stream.split();
        while let Some(message) = read.next().await {
            if let Ok(msg) = message {
                if msg.is_text() || msg.is_binary() {
                    println!("Received: {}", msg);
                    if echo {
                        if let Err(e) = write.send(msg).await {
                            eprintln!("Error sending message: {}", e);
                            break;
                        }
                    }
                } else if msg.is_close() {
                    println!("Client disconnected");
                    break;
                }
            } else {
                println!("Error receiving message");
                break;
            }
        }
    }
}