mod proxy;
mod proxy_protocol;
mod configuration;
mod mcp;

use crate::configuration::Configuration;
use crate::proxy::proxy_tcp;
use anyhow::Error;
use bytes::BufMut;
use std::str::FromStr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config_file = std::fs::File::open("./src/config.yaml")?;
    let config: Configuration = match serde_yaml::from_reader(config_file) {
        Ok(c) => c,
        Err(e) => {
            return Err(e.into());
        }
    };

    let mut handlers = Vec::new();

    for (_, proxy) in config.proxies {
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
