use crate::mcp::ping::Response;
use crate::mcp::{constants, protocol};
use bytes::BytesMut;
use log::info;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use crate::mcp::constants::HANDSHAKE;

/// Activates Fake Minecraft Server on a given address and port.
///
/// Fake Minecraft Server has 2 functionalities:
/// - Handles Server Ping. Can respond with custom MOTD and Version Text.
/// - Handles Login. If a player tries to log in, it will kick the player with a custom message.
pub async fn listen<A: ToSocketAddrs>(addr: A, response: Response) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!(
        "Fake Minecraft Server listening on {}",
        listener.local_addr()?
    );

    loop {
        let (mut client, _) = listener.accept().await?;

        let response = serde_json::to_string(&response)?;
        let response_bytes = response.as_bytes();

        let mut packet_data = BytesMut::new();
        protocol::write_varint(&mut packet_data, response_bytes.len() as i32);
        packet_data.extend_from_slice(response_bytes);

        let packet = protocol::create_packet(constants::HANDSHAKE, packet_data);

        if client.write_all(&packet).await.is_ok() {
            let _ = client.shutdown().await;
        }
    }
}

pub async fn handle_connection(client: &mut TcpStream, response: &Response) -> anyhow::Result<()> {
    let mut buf = BytesMut::new();
    client.read_buf(&mut buf).await?;

    let (length, packet_id) = protocol::read_packet(&mut buf);

    if packet_id == HANDSHAKE {
        handle_server_ping(client, response).await?;
    } else {
        info!("Unknown packet id: {}", packet_id);
    }

    Ok(())
}

async fn handle_server_ping(client: &mut TcpStream, response: &Response) -> anyhow::Result<()> {
    let response = serde_json::to_string(&response)?;
    let response_bytes = response.as_bytes();

    let mut packet_data = BytesMut::new();
    protocol::write_varint(&mut packet_data, response_bytes.len() as i32);
    packet_data.extend_from_slice(response_bytes);

    let packet = protocol::create_packet(constants::HANDSHAKE, packet_data);

    if client.write_all(&packet).await.is_ok() {
        let _ = client.shutdown().await;
    }

    Ok(())
}