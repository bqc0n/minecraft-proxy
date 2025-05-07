use crate::mcp::fake_server;
use crate::mcp::ping::Response;
use crate::proxy_protocol::{append_proxy_protocol_v2, CommandV2};
use log::{debug, error, info};
use std::net::SocketAddr;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::watch::Receiver;

pub async fn proxy_tcp(
    listen: SocketAddr,
    server: SocketAddr,
    proxy_protocol: bool,
    mut health: Receiver<bool>,
    fake_server_response: Option<Response>,
) -> io::Result<()> {
    let listener = TcpListener::bind(listen).await?;
    if proxy_protocol {
        info!(
            "Listening on {}, forwarding to {} with ProxyProtocol V2",
            listen, server
        );
    } else {
        info!("Listening on {}, forwarding to {}", listen, server);
    }

    loop {
        let (mut client, client_addr) = listener.accept().await?;
        let client_local_addr = client.local_addr()?;

        let health_changed = health.has_changed().unwrap_or(false);

        if *health.borrow_and_update() {
            debug!("Server {} is up", server);
            tokio::spawn(async move {
                let mut upstream = match TcpStream::connect(server).await {
                    Ok(upstream) => upstream,
                    Err(_) => {
                        error!("Failed to connect to server");
                        return;
                    }
                };
                if proxy_protocol {
                    let mut data = Vec::new();
                    append_proxy_protocol_v2(
                        &mut data,
                        client_addr,
                        client_local_addr,
                        CommandV2::Proxy,
                    )
                    .unwrap();
                    client.read_to_end(&mut data).await.unwrap();
                    if (upstream.write_all(&data).await).is_err() {
                        error!("Failed to write to server");
                    }
                } else if let Ok(_) = io::copy_bidirectional(&mut upstream, &mut client).await {}
            });
        } else {
            if health_changed {
                debug!("Server {} is down", server);
            }
            if let Some(ref fake_server_response) = fake_server_response {
                match fake_server::handle_connection(&mut client, fake_server_response).await {
                    Ok(_) => {}
                    Err(e) => error!("Fake Server failed to respond: {}", e),
                }
            }
        }
    }
}
