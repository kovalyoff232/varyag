use tokio::net::{TcpStream, UdpSocket};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use anyhow::{anyhow, Result};
use vyg_display::logger;

pub async fn send_tcp_request(address: &str, data: Vec<u8>, interactive: bool) -> Result<()> {
    let stream = TcpStream::connect(address).await?;
    logger::info(&format!("Connected to {}", address));

    if interactive {
        let (mut reader, mut writer) = stream.into_split();
        
        let write_task = tokio::spawn(async move {
            io::copy(&mut io::stdin(), &mut writer).await
        });

        let read_task = tokio::spawn(async move {
            io::copy(&mut reader, &mut io::stdout()).await
        });

        tokio::select! {
            res = write_task => {
                if let Ok(Err(e)) = res {
                    logger::error(&format!("Error writing to socket: {}", e));
                }
            },
            res = read_task => {
                 if let Ok(Err(e)) = res {
                    logger::error(&format!("Error reading from socket: {}", e));
                }
            },
        }

    } else {
        let mut stream = stream;
        if !data.is_empty() {
            stream.write_all(&data).await?;
        }
        
        let mut buffer = vec![0; 1024];
        match stream.read(&mut buffer).await {
            Ok(0) => {
                // Connection closed
            },
            Ok(n) => {
                io::stdout().write_all(&buffer[..n]).await?;
            },
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}

pub async fn send_udp_request(address: &str, data: Vec<u8>) -> Result<()> {
    // We need a local address to bind to. 0.0.0.0:0 is a good default.
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect(address).await?;
    
    if data.is_empty() {
        return Err(anyhow!("UDP mode requires data to send."));
    }

    socket.send(&data).await?;

    let mut buffer = vec![0; 1024];
    match socket.recv(&mut buffer).await {
        Ok(n) => {
            io::stdout().write_all(&buffer[..n]).await?;
        }
        Err(e) => {
            // This can be a timeout, which is not necessarily an error for UDP.
            // For now, we'll print it.
            logger::warn(&format!("Error receiving UDP response: {}", e));
        }
    }

    Ok(())
}
