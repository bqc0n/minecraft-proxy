use log::debug;
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
    loop {
        sleep(interval).await;

        debug!("Health check for {}", server);
        let mut stream = match time::timeout(timeout, TcpStream::connect(server)).await {
            Ok(r) => match r {
                Ok(stream) => stream,
                Err(_) => {
                    debug!("Server {} is down", server);
                    tx.send(false).unwrap();
                    continue;
                }
            },
            Err(e) => {
                debug!("Server {} is down; Timeout {}", server, e);
                tx.send(false).unwrap();
                continue;
            }
        };

        debug!("Server {} is up", server);
        // Todo: Gather Server info using Minecraft Protocol
        tx.send(true).unwrap();
    }
}
