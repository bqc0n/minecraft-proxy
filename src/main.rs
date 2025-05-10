mod configuration;
mod health_check;
mod mcp;
mod proxy;
mod proxy_protocol;

use std::path::Path;
use crate::configuration::Configuration;
use crate::mcp::ping::Response;
use crate::proxy::proxy_tcp;
use env_logger::Env;
use log::{error, info};
use toml::de::Error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = Env::default().filter_or("RUST_LOG", "debug");
    env_logger::init_from_env(env);

    let args: Vec<String> = std::env::args().collect();
    let config_file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "./config.toml".to_string()
    };

    info!("Loading configuration from {}", config_file_path);

    if !Path::new(&config_file_path).exists() {
        error!("Configuration file {} does not exist", config_file_path);
        return Err(anyhow::anyhow!("Configuration file does not exist"));
    }

    let config_str = std::fs::read_to_string(&config_file_path)?;
    let config: Configuration = match toml::from_str::<Configuration>(&config_str) {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to parse configuration file: {}", e);
            return Err(anyhow::anyhow!("Failed to parse configuration file: {}", e));
        }
    };

    let mut handlers = Vec::new();

    if config.health_check.enabled {
        info!(
            "Health check enabled with {}s interval and {}s timeout",
            config.health_check.interval().as_secs(),
            config.health_check.timeout().as_secs()
        );
    }

    for proxy in config.proxies {
        let (tx, rx) = tokio::sync::watch::channel(true);
        for bind_addr in proxy.bind {
            handlers.push(tokio::spawn(proxy_tcp(
                bind_addr,
                proxy.host,
                proxy.proxy_protocol,
                rx.clone(),
                config.sorry_server.clone(),
            )));
        }
        if config.health_check.enabled {
            handlers.push(tokio::spawn(health_check::activate_health_check_for(
                proxy.host,
                tx,
                config.health_check.interval(),
                config.health_check.timeout(),
            )))
        }
    }

    for handler in handlers {
        if let Err(e) = handler.await {
            error!("Error: {:?}", e);
        }
    }

    Ok(())
}
