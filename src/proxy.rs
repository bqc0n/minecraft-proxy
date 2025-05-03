use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use tokio::{io, select};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use crate::proxy_protocol::{append_proxy_protocol_v2, CommandV2};

pub async fn proxy_tcp_v4(listen: SocketAddrV4, server: SocketAddr, proxy_protocol: bool) -> io::Result<()> {
    let listener = TcpListener::bind(listen).await?;
    loop {
        let (mut client, client_addr) = listener.accept().await?;
        let client_local_addr = client.local_addr()?;

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
    }
}

