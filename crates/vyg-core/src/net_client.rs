use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use anyhow::Result;

pub async fn send_tcp_request(address: &str, data: Vec<u8>) -> Result<()> {
    let mut stream = TcpStream::connect(address).await?;
    stream.write_all(&data).await?;

    let mut buffer = vec![0; 1024];
    let n = stream.read(&mut buffer).await?;
    let response = &buffer[..n];

    io::stdout().write_all(response).await?;

    Ok(())
}
