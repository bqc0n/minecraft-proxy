use crate::configuration::SorryServerConfig;
use crate::mcp::constants::{VARINT_CONTINUE_BIT, VARINT_SEGMENT_BITS};
use crate::mcp::ping::Response;
use crate::mcp::{constants, fake_server};
use bytes::{Buf, BytesMut};
use serde_json::{json, Value};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

// todo
