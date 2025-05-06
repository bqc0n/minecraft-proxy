mod proxy;
mod proxy_protocol;
mod configuration;
mod mcp;
mod health_check;

use crate::configuration::Configuration;
use crate::proxy::proxy_tcp;
use anyhow::Error;
use bytes::BufMut;
use std::str::FromStr;
use std::time::Duration;
use env_logger::Env;
use log::error;
use serde::Serialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::mcp::{fake_server, ping};
use crate::mcp::ping::Response;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = Env::default()
        .filter_or("RUST_LOG", "info");
    env_logger::init_from_env(env);

    let config_file = std::fs::File::open("./src/config.yaml")?;
    let config: Configuration = match serde_yaml::from_reader(config_file) {
        Ok(c) => c,
        Err(e) => {
            return Err(e.into());
        }
    };

    let mut handlers = Vec::new();
    let response: Option<Response> = if let Some(ss_config) = config.sorry_server {
        Some(Response::from_config(ss_config))
    } else {
        None
    };

    for (_, proxy) in config.proxies {
        for bind_addr in proxy.bind {
            let (tx, rx) = tokio::sync::watch::channel(true);
            handlers.push(tokio::spawn(proxy_tcp(bind_addr, proxy.server, proxy.proxy_protocol, rx, response.clone())));
            handlers.push(tokio::spawn(
                health_check::activate_health_check_for(proxy.server, tx, Duration::from_secs(5), Duration::from_secs(2)),
            ))
        }
    }

    for handler in handlers {
        if let Err(e) = handler.await {
            error!("Error: {:?}", e);
        }
    }

    Ok(())
}
