use anyhow::{anyhow, Result};
use tokio::process::Command;
use std::process::Stdio;
use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::sync::oneshot;

pub async fn start_tunnel(
    local_port: u16,
    remote_port: u16,
    server: &str,
) -> Result<(String, tokio::task::JoinHandle<()>)> {
    let mut cmd = Command::new("bore");
    cmd.arg("local")
        .arg(local_port.to_string())
        .arg("--to")
        .arg(server);
    
    if remote_port > 0 {
        cmd.arg("--remote-port").arg(remote_port.to_string());
    }

    cmd.stdout(Stdio::piped()).kill_on_drop(true);

    let mut child = cmd.spawn()?;
    let stdout = child.stdout.take().ok_or_else(|| anyhow!("Failed to capture stdout"))?;

    let (tx, rx) = oneshot::channel();

    let handle = tokio::spawn(async move {
        let mut reader = BufReader::new(stdout).lines();
        let mut tx_opt = Some(tx);

        while let Ok(Some(line)) = reader.next_line().await {
            println!("[bore output] {}", line);
            if line.contains("listening at") {
                if let Some(address) = line.split("listening at ").last() {
                    if let Some(tx) = tx_opt.take() {
                        if tx.send(address.to_string()).is_err() {
                            eprintln!("Failed to send address to test");
                        }
                    }
                }
            }
        }
        if let Err(e) = child.wait().await {
            eprintln!("bore-cli process exited with error: {}", e);
        }
    });

    let address = rx.await?;
    Ok((address, handle))
}
