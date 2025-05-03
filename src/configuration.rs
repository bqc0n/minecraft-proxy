use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(crate) struct Configuration {
    #[serde(rename = "servers")]
    pub proxies: HashMap<String, ProxyConfig>,
    pub sorry_server: SorryServer,
}

#[derive(Deserialize, Debug)]
pub struct ProxyConfig {
    #[serde(default = "default_bind")]
    pub bind: Vec<SocketAddr>,
    #[serde(deserialize_with = "deserialize_listen")]
    pub server: SocketAddr,
    #[serde(default)] // Default to false
    pub proxy_protocol: bool,
}

#[derive(Deserialize)]
pub struct SorryServer {

}

fn deserialize_listen<'de, D>(deserializer: D) -> Result<SocketAddr, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
   let sock_addr = s.to_socket_addrs()
       .map_err(serde::de::Error::custom)?.collect::<Vec<_>>().get(0).cloned();
    match sock_addr {
        Some(addr) => Ok(addr),
        None => Err(serde::de::Error::custom("Invalid socket address")),
    }
}

fn default_bind() -> Vec<SocketAddr> {
    vec!["0.0.0.0:25565".parse().unwrap(), "[::]:25565".parse().unwrap()]
}