use std::time::Duration;
use anyhow::Result;
use tokio::process::Command;
use tokio::time::sleep;
use vyg_core::tunnel_client;
use reqwest;
use portpicker;

async fn start_test_server(port: u16) {
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();
    let app = axum::Router::new().route("/", axum::routing::get(|| async { "Hello, Varyag!" }));
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Test server failed: {}", e);
    }
}

#[tokio::test]
#[ignore] // This test is ignored by default because it depends on `bore` being in the PATH.
async fn test_bridge_e2e_with_command() -> Result<()> {
    // This test requires `bore` to be installed and available in the system's PATH.
    // You can install it with `cargo install bore-cli`.
    if let Err(_) = Command::new("bore").arg("--version").output().await {
        println!("Skipping bridge test: `bore` command not found in PATH.");
        return Ok(());
    }

    let local_port = portpicker::pick_unused_port().expect("No free ports available");
    let expected_response = "Hello, Varyag!";

    // 1. Start a local server in the background.
    tokio::spawn(start_test_server(local_port));
    sleep(Duration::from_millis(500)).await;

    // 2. Start the tunnel client.
    let (public_address, tunnel_handle) = tunnel_client::start_tunnel(local_port, 0, "bore.pub").await?;
    
    // 3. Make a request to the public URL.
    let url = format!("http://{}", public_address);
    println!("Making request to public URL: {}", url);
    
    // Retry logic for the request, as the tunnel might take a moment to be ready.
    let mut response = None;
    for _ in 0..5 {
        sleep(Duration::from_secs(1)).await;
        match reqwest::get(&url).await {
            Ok(res) => {
                response = Some(res);
                break;
            }
            Err(e) => {
                println!("Request failed, retrying... Error: {}", e);
            }
        }
    }

    let response = response.ok_or_else(|| anyhow::anyhow!("Failed to get a response from the tunnel"))?;
    
    // 4. Assert the response is correct.
    assert!(response.status().is_success());
    let body = response.text().await?;
    assert_eq!(body, expected_response);
    println!("Successfully received response from local server through tunnel.");

    // 5. Clean up.
    tunnel_handle.abort();

    Ok(())
}