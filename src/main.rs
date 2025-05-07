mod configuration;
mod health_check;
mod mcp;
mod proxy;
mod proxy_protocol;

use crate::configuration::Configuration;
use crate::mcp::ping::Response;
use crate::proxy::proxy_tcp;
use env_logger::Env;
use log::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = Env::default().filter_or("RUST_LOG", "debug");
    env_logger::init_from_env(env);

    let config_file = std::fs::File::open("./src/config.yaml")?;
    let config: Configuration = match serde_yaml::from_reader(config_file) {
        Ok(c) => c,
        Err(e) => {
            return Err(e.into());
        }
    };

    let mut handlers = Vec::new();
    let response: Option<Response> = config.sorry_server.map(Response::from_config);

    if config.health_check.enabled {
        info!(
            "Health check enabled with {}s interval and {}s timeout",
            config.health_check.interval().as_secs(),
            config.health_check.timeout().as_secs()
        );
    }

    for (_, proxy) in config.proxies {
        for bind_addr in proxy.bind {
            let (tx, rx) = tokio::sync::watch::channel(true);
            handlers.push(tokio::spawn(proxy_tcp(
                bind_addr,
                proxy.server,
                proxy.proxy_protocol,
                rx,
                response.clone(),
            )));
            if config.health_check.enabled {
                handlers.push(tokio::spawn(health_check::activate_health_check_for(
                    proxy.server,
                    tx,
                    config.health_check.interval(),
                    config.health_check.timeout(),
                )))
            }
        }
    }

    for handler in handlers {
        if let Err(e) = handler.await {
            error!("Error: {:?}", e);
        }
    }

    Ok(())
}
