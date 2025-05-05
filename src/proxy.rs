use crate::proxy_protocol::{append_proxy_protocol_v2, CommandV2};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::io;
use tokio::sync::watch::Receiver;
use crate::mcp::fake_server;
use crate::mcp::ping::Response;

pub async fn proxy_tcp(
    listen: SocketAddr,
    server: SocketAddr,
    proxy_protocol: bool,
    mut health: Receiver<bool>,
    fake_server_response: Option<Response>,
) -> io::Result<()> {
    let listener = TcpListener::bind(listen).await?;
    if proxy_protocol {
        println!("Listening on {}, forwarding to {} with PP", listen, server);
    } else {
        println!("Listening on {}, forwarding to {}", listen, server);
    }

    loop {
        let (mut client, client_addr) = listener.accept().await?;
        let client_local_addr = client.local_addr()?;

        let health_changed = health.has_changed().unwrap_or(false);

        if *health.borrow_and_update() {
            println!("Server {} is up", server);
            tokio::spawn(async move {
                let mut upstream = match TcpStream::connect(server).await {
                    Ok(upstream) => upstream,
                    Err(_) => {
                        eprintln!("Failed to connect to server");
                        return
                    },
                };
                if proxy_protocol {
                    let mut data = Vec::new();
                    append_proxy_protocol_v2(&mut data, client_addr, client_local_addr, CommandV2::Proxy).unwrap();
                    client.read_to_end(&mut data).await.unwrap();
                    if let Err(_) = upstream.write_all(&data).await {
                        eprintln!("Failed to write to server");
                        return;
                    }
                } else {
                    match io::copy_bidirectional(&mut upstream, &mut client).await {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                }
            });
        } else {
            if health_changed {
                println!("Server {} is down", server);
            }
            if let Some(ref fake_server_response) = fake_server_response {
                match fake_server::respond(&mut client, fake_server_response).await {
                    Ok(_) => {}
                    Err(e) => println!("Fake Server failed to respond: {}", e),
                }
            }
        }
    }
}