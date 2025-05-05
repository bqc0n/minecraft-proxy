use bytes::{Buf, BytesMut};
use serde_json::{json, Value};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use crate::configuration::SorryServerConfig;
use crate::mcp::{constants, fake_server};
use crate::mcp::constants::{VARINT_CONTINUE_BIT, VARINT_SEGMENT_BITS};
use crate::mcp::ping::Response;

#[tokio::test]
async fn test_fake_server_status() -> anyhow::Result<()> {
    let config = SorryServerConfig {
        version: "test".to_string(),
        motd: vec!["test-motd".to_string()],
        kick_message: vec!["test-kick".to_string()],
    };
    
    let result_json = json!({
        "version": {
            "name": config.version,
            "protocol": constants::DEFAULT_PROTOCOL,
        },
        "players": {
            "max": 0,
            "online": 0,
            "sample": [],
        },
        "description": {
            "text": config.motd.join("\n"),
        },
    });
    
    let response = Response::from_config(config);

    tokio::spawn(async move {
        fake_server::listen("127.0.0.1:25565", response).await.unwrap();
    });
    
    let mut con = TcpStream::connect("127.0.0.1:25565").await?;

    let mut buf = BytesMut::with_capacity(1024);
    con.read_buf(&mut buf).await?;
    
    let _packet_length = read_varint(&mut buf)?;
    let packet_id = read_varint(&mut buf)?;
    
    let _json_length = read_varint(&mut buf)?;
    let json_str = String::from_utf8(buf.to_vec()).map_err(|_| anyhow::anyhow!("Invalid UTF-8"))?;
    let response_received: Value = serde_json::from_str(&json_str).map_err(|_| anyhow::anyhow!("Invalid JSON"))?;

    assert_eq!(packet_id, constants::HANDSHAKE as u32);
    assert_eq!(response_received, result_json);
    
    Ok(())
}

fn read_varint(buf: &mut BytesMut) -> anyhow::Result<u32> {
    let mut value = 0;
    let mut pos = 0;

    loop {
        let byte = buf.get_u8();
        value |= ((byte & VARINT_SEGMENT_BITS) as u32) << pos;
        if byte & VARINT_CONTINUE_BIT == 0 {
            break;
        }
        pos += 7;
        if pos >= 32 { 
            return Err(anyhow::anyhow!("Varint is too big"));
        }
    }

    Ok(value)
}