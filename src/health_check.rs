use log::{debug, info};
use std::net::SocketAddr;
use std::time::Duration;
use anyhow::anyhow;
use tokio::net::TcpStream;
use tokio::sync::watch::Sender;
use tokio::time::sleep;
use tokio::{io, time};

pub(crate) async fn activate_health_check_for(
    server: SocketAddr,
    tx: Sender<bool>,
    interval: Duration,
    timeout: Duration,
) -> anyhow::Result<()> {
    let mut init = true;
    let mut history_is_healthy = false;

    loop {
        let healthy = check(&server, timeout).await;

        match healthy {
            Ok(_) => {
                if init || !history_is_healthy {
                    info!("Server {} is up", server);
                    history_is_healthy = true;
                    init = false;
                }
                tx.send(true)?;
            }
            Err(e) => {
                if init || history_is_healthy {
                    info!("Server {} is down: {}", server, e);
                    history_is_healthy = false;
                    init = false;
                }
                tx.send(false)?;
            }
        }
    }
}

/// Returns `Err` if unhealthy, `Ok` if healthy
async fn check(
    server: &SocketAddr,
    timeout: Duration,
) -> anyhow::Result<()> {
    if let Ok(r) = time::timeout(timeout, TcpStream::connect(server)).await {
        match r {
            Ok(s) => s,
            Err(e) => return Err(anyhow!(e)),
        }
    } else {
        return Err(anyhow!("Timeout"));
    };

    // Todo: Gather Server info using Minecraft Protocol
    Ok(())
}