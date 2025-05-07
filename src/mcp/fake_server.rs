use crate::mcp::ping::Response;
use crate::mcp::protocol::{ClientBoundMcPacket, HandshakeState, ServerBoundMcPacket};
use log::{debug, error};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn handle_connection(client: &mut TcpStream, response: &Response, kick_msg: &Vec<String>) -> anyhow::Result<()> {
    let packet = ServerBoundMcPacket::read_packet(client).await?;
    match packet {
        ServerBoundMcPacket::Handshake { next_state, .. } => match next_state {
            HandshakeState::Status => handle_server_ping(client, response).await?,
            HandshakeState::Login => handle_login(client, kick_msg).await?,
            HandshakeState::Transfer => {
                return Err(anyhow::anyhow!("Transfer state is not supported"))
            }
        },
    }
    Ok(())
}

async fn handle_server_ping(client: &mut TcpStream, response: &Response) -> anyhow::Result<()> {
    let packet = ClientBoundMcPacket::status_response(response).to_packet();

    match client.write_all(&packet).await {
        Ok(_) => client.shutdown().await?,
        Err(e) => error!("Failed to send a packet: {}", e),
    }

    Ok(())
}

async fn handle_login(client: &mut TcpStream, kick_message: &Vec<String>) -> anyhow::Result<()> {
    let packet = ClientBoundMcPacket::login_disconnect(kick_message).to_packet();

    match client.write_all(&packet).await {
        Ok(_) => client.shutdown().await?,
        Err(e) => error!("Failed to send a packet: {}", e),
    }

    Ok(())
}