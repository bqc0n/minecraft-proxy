mod proxy;
mod proxy_protocol;
mod configuration;
mod mcp;

use crate::configuration::Configuration;
use crate::proxy::proxy_tcp;
use anyhow::Error;
use bytes::BufMut;
use std::str::FromStr;
use serde::Serialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::mcp::fake_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_file = std::fs::File::open("./src/config.yaml")?;
    let config: Configuration = match serde_yaml::from_reader(config_file) {
        Ok(c) => c,
        Err(e) => {
            return Err(e.into());
        }
    };

    let resp = mcp::ping::Response::from_config(config.sorry_server.unwrap());
    fake_server::listen("127.0.0.1:25565", resp).await?;

    //
    // let mut handlers = Vec::new();
    //
    // for (_, proxy) in config.proxies {
    //     for bind_addr in proxy.bind {
    //         handlers.push(tokio::spawn(proxy_tcp(bind_addr, proxy.server, proxy.proxy_protocol)))
    //     }
    // }
    //
    // for handler in handlers {
    //     if let Err(e) = handler.await {
    //         eprintln!("Error: {:?}", e);
    //     }
    // }
    //
    Ok(())
}
