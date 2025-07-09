use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use anyhow::Result;
use axum::{
    extract::State,
    routing::{any, get_service},
    Router,
    http::{Request, StatusCode},
    body::Body,
    response::{IntoResponse, Response},
};
use std::net::SocketAddr;
use http_body_util::BodyExt;
use tokio::net::TcpListener;
use futures_util::{StreamExt, SinkExt};
use std::path::PathBuf;
use tower_http::services::ServeDir;
use reqwest::Client;
use std::sync::Arc;
use vyg_display::logger;

pub async fn start_tcp_listener(port: u16, echo: bool) -> Result<()> {
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    logger::info(&format!("Listening for TCP on port {}", port));

    loop {
        let (mut socket, addr) = listener.accept().await?;
        logger::info(&format!("Accepted connection from {}", addr));

        if echo {
            tokio::spawn(async move {
                let (mut reader, mut writer) = socket.split();
                if let Err(e) = io::copy(&mut reader, &mut writer).await {
                    logger::error(&format!("Failed to copy data: {}", e));
                }
            });
        } else {
            tokio::spawn(async move {
                let mut buffer = vec![0; 1024];
                loop {
                    match socket.read(&mut buffer).await {
                        Ok(0) => {
                            logger::info(&format!("Connection closed by {}", addr));
                            return;
                        }
                        Ok(n) => {
                            let data = &buffer[..n];
                            if let Err(e) = io::stdout().write_all(data).await {
                                logger::error(&format!("Failed to write to stdout: {}", e));
                                return;
                            }
                        }
                        Err(e) => {
                            logger::error(&format!("Failed to read from socket: {}", e));
                            return;
                        }
                    }
                }
            });
        }
    }
}

pub async fn start_http_listener(port: u16, serve_path: Option<PathBuf>, proxy_pass: Option<String>) -> Result<()> {
    let app = if let Some(path) = serve_path {
        logger::info(&format!("Serving files from {:?}", path));
        Router::new().nest_service("/", get_service(ServeDir::new(path)))
    } else if let Some(proxy_url) = proxy_pass {
        logger::info(&format!("Proxying requests to {}", proxy_url));
        let client = Client::new();
        let state = AppState { client, proxy_url };
        Router::new().fallback(proxy_handler).with_state(Arc::new(state))
    } else {
        logger::info("Using inspect_request handler");
        Router::new().route("/*path", any(inspect_request))
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    logger::info(&format!("Listening for HTTP on port {}", port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
struct AppState {
    client: Client,
    proxy_url: String,
}

async fn proxy_handler(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
) -> Result<Response, StatusCode> {
    let path_and_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or_else(|| req.uri().path());

    let target_url = format!("{}{}", state.proxy_url, path_and_query);

    logger::info(&format!("Proxying {} {} to {}", req.method(), path_and_query, target_url));

    let (parts, body) = req.into_parts();
    let body_bytes = body.collect().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?.to_bytes();

    let client_req = state.client.request(parts.method, &target_url)
        .headers(parts.headers)
        .body(body_bytes)
        .build()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let res = state.client.execute(client_req).await.map_err(|e| {
        logger::error(&format!("Proxy error: {}", e));
        StatusCode::BAD_GATEWAY
    })?;

    let mut response_builder = Response::builder().status(res.status());
    for (key, value) in res.headers() {
        response_builder = response_builder.header(key, value);
    }
    
    let response = response_builder
        .body(Body::from(res.bytes().await.unwrap_or_default()))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(response)
}


async fn inspect_request(req: Request<Body>) -> impl IntoResponse {
    logger::info(&format!("Request: {} {}", req.method(), req.uri()));

    logger::info("Headers:");
    for (k, v) in req.headers() {
        logger::info(&format!("{}: {}", k.as_str(), v.to_str().unwrap_or("")));
    }

    let body_bytes = req.into_body().collect().await.unwrap().to_bytes();
    if let Ok(body_str) = String::from_utf8(body_bytes.to_vec()) {
        if !body_str.is_empty() {
            logger::info(&format!("Body: {}", body_str));
        }
    } else {
        logger::info("Body: (non-UTF8 data)");
    }
    logger::info("---");

    (StatusCode::OK, "Request logged to console")
}

pub async fn start_ws_listener(port: u16, echo: bool) -> Result<()> {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    logger::info(&format!("Listening for WebSocket on port {}", port));

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
                    logger::info(&format!("Received: {}", msg));
                    if echo {
                        if let Err(e) = write.send(msg).await {
                            logger::error(&format!("Error sending message: {}", e));
                            break;
                        }
                    }
                } else if msg.is_close() {
                    logger::info("Client disconnected");
                    break;
                }
            } else {
                logger::error("Error receiving message");
                break;
            }
        }
    }
}
