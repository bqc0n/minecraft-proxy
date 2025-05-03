mod proxy;
mod proxy_protocol;
mod configuration;

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::json;
use bytes::{BufMut, BytesMut};
use serde_yaml::Error;
use crate::configuration::Configuration;
use crate::proxy::{proxy_tcp_v4};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config_file = std::fs::File::open("./src/config.yaml")?;
    let config: Configuration = match serde_yaml::from_reader(config_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to parse the config file: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to parse the config file"));
        }
    };
    
    let mut handlers = Vec::new();

    for (key, proxy) in config.proxies {
        for bind in proxy.bind {
            match bind {
                SocketAddr::V4(addr) => {
                    handlers.push(tokio::spawn(proxy_tcp_v4(addr, proxy.server, proxy.proxy_protocol)))
                }
                SocketAddr::V6(addr) => {
                    
                }
            }
        }    
    }
    
    for handler in handlers {
        if let Err(e) = handler.await {
            eprintln!("Error: {:?}", e);
        }
    }

    Ok(())
}

// MinecraftのVarInt形式で整数を書き込む
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
