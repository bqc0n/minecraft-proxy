use bytes::{BufMut, BytesMut};
use serde_json::json;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, ToSocketAddrs};
use crate::mcp::{constants, protocol};
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

        println!("Fake Minecraft Server accepted from {}", client.peer_addr()?);
        
        let response = serde_json::to_string(&response)?;
        let response_bytes = response.as_bytes();

        let packet = protocol::create_packet(constants::HANDHSAKE, response_bytes);

        if client.write_all(&packet).await.is_ok() {
            let _ = client.shutdown().await;
        }
    }
}

fn write_varint(buf: &mut BytesMut, mut value: i32) {
    loop {
        if (value & !0x7F) == 0 {
            buf.put_u8(value as u8);
            return;
        } else {
            buf.put_u8(((value & 0x7F) | 0x80) as u8);
            value >>= 7;
        }
    }
}