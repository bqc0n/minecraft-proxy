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
use crate::proxy::{proxy_tcp};

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
        for bind_addr in proxy.bind {
            handlers.push(tokio::spawn(proxy_tcp(bind_addr, proxy.server, proxy.proxy_protocol)))
        }    
    }
    
    for handler in handlers {
        if let Err(e) = handler.await {
            eprintln!("Error: {:?}", e);
        }
    }

    Ok(())
}
