use clap::Args;
use vyg_core::tunnel_client;

#[derive(Args, Debug)]
pub struct BridgeCommand {
    /// The local port to expose.
    #[arg()]
    pub local_port: u16,

    /// The port on the remote server to listen on.
    #[arg(short, long, default_value_t = 0)]
    pub remote_port: u16,

    /// Address of the remote server to connect to.
    #[arg(short, long, default_value = "bore.pub")]
    pub server: String,
}

pub async fn handle_bridge(command: BridgeCommand) {
    match tunnel_client::start_tunnel(
        command.local_port,
        command.remote_port,
        &command.server,
    )
    .await
    {
        Ok((_address, handle)) => {
            if let Err(e) = handle.await {
                eprintln!("Tunnel task failed: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error starting tunnel: {}", e);
        }
    }
}
