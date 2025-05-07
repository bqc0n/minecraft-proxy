use log::{debug, info, warn};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::watch::Sender;
use tokio::time::sleep;
use tokio::{io, time};

pub(crate) async fn activate_health_check_for(
    server: SocketAddr,
    tx: Sender<bool>,
    interval: Duration,
    timeout: Duration,
) -> io::Result<()> {
    let mut history_is_healthy = true;

    loop {
        sleep(interval).await;

        debug!("Health check for {}", server);
        let stream = if let Ok(r) = time::timeout(timeout, TcpStream::connect(server)).await {
            match r {
                Ok(s) => {
                    if !history_is_healthy {
                        info!("Server {} is up", server);
                    }
                    s
                }
                Err(_) => {
                    if history_is_healthy {
                        info!("Server {} is down", server);
                        tx.send(false).unwrap();
                        history_is_healthy = false;
                    }
                    continue;
                }
            }
        } else {
            if history_is_healthy {
                info!("Server {} is down: Timeout", server);
                tx.send(false).unwrap();
                history_is_healthy = false;
            }
            continue;
        };

        // Todo: Gather Server info using Minecraft Protocol
        tx.send(true).unwrap();
    }
}
