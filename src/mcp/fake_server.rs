use serde_json::json;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, ToSocketAddrs};
use crate::mcp::ping::Response;

/// Activates Fake Minecraft Server on a given address and port.
///
/// Fake Minecraft Server has 2 functionalities:
/// - Handles Server Ping. Can respond with custom MOTD and Version Text.
/// - Handles Login. If a player tries to log in, it will kick the player with a custom message.
pub async fn listen<A: ToSocketAddrs>(addr: A, response: Response) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    println!("Fake Minecraft Server listening on {}", listener.local_addr()?);

    loop {
        let (mut client, _) = listener.accept().await?;
        
        let response = serde_yaml::to_string(&response)?;
        client.write_all(response.as_bytes()).await?; 
    }
}