use crate::mcp::constants;
use crate::mcp::constants::{
    VARINT_CONTINUE_BIT, VARINT_CONTINUE_BIT_I32, SEVEN_BITS, SEVEN_BITS_I32,
};
use crate::mcp::ping::Response;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use log::debug;
use serde_json::json;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub enum ServerBoundMcPacket {
    Handshake {
        protocol_version: McVarInt,
        server_address: String,
        server_port: u16,
        next_state: HandshakeState,
    },
}

impl ServerBoundMcPacket {
    pub async fn read_packet(client: &mut TcpStream) -> anyhow::Result<Self> {
        let length = McVarInt::read_stream(client).await?.int() as usize;
        let mut buf = BytesMut::with_capacity(length);
        buf.resize(length, 0);
        client.read_exact(&mut buf).await?;

        let packet_id = McVarInt::read(&mut buf)?.int();

        if packet_id == constants::HANDSHAKE {
            let protocol_version = McVarInt::read_i32(&mut buf)?;
            let server_address = McString::read_string(&mut buf)?;
            let server_port = buf.get_u16();
            let next_state = HandshakeState::from(McVarInt::read(&mut buf)?);
            debug!("Handshake packet: protocol_version: {}, server_address: {}, server_port: {}, next_state: {:?}", protocol_version, server_address, server_port, next_state);

            Ok(ServerBoundMcPacket::Handshake {
                protocol_version: McVarInt(protocol_version),
                server_address,
                server_port,
                next_state,
            })
        } else {
            Err(anyhow::anyhow!("packet id {} isn't supported.", packet_id))
        }
    }
}

pub enum ClientBoundMcPacket {
    StatusResponse { json_response: String },
    LoginDisconnect { reason: String },
}

impl ClientBoundMcPacket {
    pub fn status_response(response: &Response) -> Self {
        let json_response = serde_json::to_string(response).unwrap();
        ClientBoundMcPacket::StatusResponse { json_response }
    }

    pub fn login_disconnect(reason: &Vec<String>) -> Self {
        let reason = reason.join("\n");
        ClientBoundMcPacket::LoginDisconnect { reason }
    }

    pub fn to_packet(&self) -> Bytes {
        let mut packet_data = BytesMut::new();
        // the entire packet len contains the len of Packet Id: VarInt
        // so we put the packet_id to the packet_data BytesMut
        match self {
            ClientBoundMcPacket::StatusResponse { json_response } => {
                McVarInt(constants::HANDSHAKE).write(&mut packet_data);
                McVarInt(json_response.len() as i32).write(&mut packet_data);
                packet_data.extend_from_slice(json_response.as_bytes());
            }
            ClientBoundMcPacket::LoginDisconnect { reason } => {
                McVarInt(constants::HANDSHAKE).write(&mut packet_data);
                let data = json!({ "text": reason }).to_string();
                McVarInt(data.len() as i32).write(&mut packet_data);
                packet_data.extend_from_slice(data.as_bytes());
            }
        };

        let mut packet = BytesMut::new();
        McVarInt(packet_data.len() as i32).write(&mut packet);
        packet.extend(packet_data);
        packet.freeze()
    }
}

#[derive(Debug)]
pub enum HandshakeState {
    Status = 1,
    Login = 2,
    Transfer = 3,
}

impl HandshakeState {
    pub fn from(value: McVarInt) -> Self {
        match value.int() {
            1 => HandshakeState::Status,
            2 => HandshakeState::Login,
            3 => HandshakeState::Transfer,
            _ => panic!("Invalid handshake state"),
        }
    }
}

pub struct McVarInt(i32);

impl McVarInt {
    pub fn new(value: i32) -> Self {
        McVarInt(value)
    }

    pub fn read(buf: &mut BytesMut) -> anyhow::Result<Self> {
        let mut value = 0i32;
        let mut pos = 0i32;

        loop {
            let byte = buf.get_u8();
            value |= ((byte & SEVEN_BITS) as i32) << pos;
            if byte & VARINT_CONTINUE_BIT == 0 {
                break;
            }
            pos += 7;
            if pos >= 32 {
                return Err(anyhow::anyhow!("Varint is too big"));
            }
        }

        Ok(McVarInt(value))
    }

    pub async fn read_stream<R: AsyncReadExt + Unpin>(reader: &mut R) -> anyhow::Result<Self> {
        let mut value = 0i32;
        let mut pos = 0i32;

        loop {
            let byte = reader.read_u8().await?;
            value |= ((byte & SEVEN_BITS) as i32) << pos;
            if byte & VARINT_CONTINUE_BIT == 0 {
                break;
            }
            pos += 7;
            if pos >= 32 {
                return Err(anyhow::anyhow!("Varint is too big"));
            }
        }

        Ok(McVarInt(value))
    }

    pub fn write(&self, buf: &mut BytesMut) -> u8 {
        let mut length = 0;
        let mut value = self.int();

        loop {
            length += 1;
            if (value & !SEVEN_BITS_I32) == 0 {
                buf.put_u8(value as u8);
                return length;
            } else {
                buf.put_u8(((value & SEVEN_BITS_I32) | VARINT_CONTINUE_BIT_I32) as u8);
                value >>= 7;
            }
        }
    }

    pub fn read_i32(buf: &mut BytesMut) -> anyhow::Result<i32> {
        Ok(Self::read(buf)?.0)
    }

    pub fn int(&self) -> i32 {
        self.0
    }
}

pub struct McString {
    pub length: McVarInt,
    pub value: String,
}

impl McString {
    pub fn read(buf: &mut BytesMut) -> anyhow::Result<Self> {
        let length = McVarInt::read(buf)?;
        let string_data = buf.split_to(length.int() as usize);
        let value = String::from_utf8(Vec::from(string_data.as_ref()))?;
        Ok(McString { length, value })
    }

    pub fn read_string(buf: &mut BytesMut) -> anyhow::Result<String> {
        Ok(Self::read(buf)?.value)
    }
}
