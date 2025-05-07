use crate::mcp::ping::Response;
use crate::mcp::{constants, protocol};
use bytes::BytesMut;
use log::{error, info};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use crate::mcp::constants::HANDSHAKE;
use crate::mcp::protocol::{ClientBoundMcPacket, HandshakeState, ServerBoundMcPacket};

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
    }
}

pub async fn handle_connection(client: &mut TcpStream, response: &Response) -> anyhow::Result<()> {
    let mut buf = Vec::new();
    client.read_to_end(&mut buf).await?;
    let packet = protocol::ServerBoundMcPacket::read(&buf)?;
    match packet {
        ServerBoundMcPacket::Handshake { next_state, .. } => {
            match next_state {
                HandshakeState::Status => handle_server_ping(client, response).await?,
                HandshakeState::Login => return Err(anyhow::anyhow!("Login state is not supported")),
                HandshakeState::Transfer => return Err(anyhow::anyhow!("Transfer state is not supported")),

            }
        }
    }
    Ok(())
}

async fn handle_server_ping(client: &mut TcpStream, response: &Response) -> anyhow::Result<()> {
    let packet = ClientBoundMcPacket::status_response(response)
        .to_packet();

    println!("Packet: {:?}", packet);

    match client.write_all(&packet).await {
        Ok(_) => client.shutdown().await?,
        Err(e) => error!("Failed to send a packet: {}", e),
    }

    Ok(())
}