use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use url::Url;
use anyhow::Result;

pub async fn connect_ws(url: &str, message: Option<String>) -> Result<()> {
    let url = Url::parse(url)?;
    let (ws_stream, _) = connect_async(url).await?;
    println!("WebSocket handshake has been successfully completed");

    let (mut write, mut read) = ws_stream.split();

    if let Some(msg) = message {
        write.send(Message::Text(msg)).await?;
    }

    while let Some(msg) = read.next().await {
        let msg = msg?;
        match msg {
            Message::Text(t) => {
                println!("Received: {}", t);
            }
            Message::Binary(b) => {
                println!("Received binary: {:?}", b);
            }
            Message::Ping(_) => { /* tungstenite handles this */ }
            Message::Pong(_) => { /* tungstenite handles this */ }
            Message::Close(_) => {
                println!("Connection closed");
                break;
            }
            Message::Frame(_) => { /* Should not happen */ }
        }
    }

    Ok(())
}
