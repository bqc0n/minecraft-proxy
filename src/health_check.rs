use std::net::SocketAddr;
use std::time::Duration;
use tokio::sync::watch::Sender;

pub(crate) async fn activate_health_check_for(server: SocketAddr, tx: Sender<bool>, interval: Duration) -> tokio::io::Result<()> {
    loop {
        println!("Health check for {}", server);
        let mut stream = match tokio::net::TcpStream::connect(server).await {
            Ok(stream) => stream,
            Err(_) => {
                println!("Server {} is down", server);
                tx.send(false).unwrap();
                tokio::time::sleep(interval).await;
                continue;
            }
        };

        println!("Server {} is up", server);
        // Todo: Gather Server info using Minecraft Protocol
        tx.send(true).unwrap();

        tokio::time::sleep(interval).await;
    }
}