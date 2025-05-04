use serde::Deserialize;
use std::collections::HashMap;
use std::net::{SocketAddr, ToSocketAddrs};
use std::ops::Add;

#[derive(Deserialize)]
pub(crate) struct Configuration {
    #[serde(rename = "servers")]
    pub proxies: HashMap<String, ProxyConfig>,
    pub sorry_server: Option<SorryServerConfig>,
}

#[derive(Deserialize, Debug)]
pub struct ProxyConfig {
    #[serde(default = "default_bind")]
    pub bind: Vec<SocketAddr>,
    #[serde(deserialize_with = "deserialize_server")]
    pub server: SocketAddr,
    #[serde(default)] // Default to false
    pub proxy_protocol: bool,
}

#[derive(Deserialize)]
pub(crate) struct SorryServerConfig {
    pub(crate) version: String,
    pub(crate) motd: Vec<String>,
    pub(crate) kick_message: Vec<String>,
}

fn deserialize_server<'de, D>(deserializer: D) -> Result<SocketAddr, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let sock_addr = match s.to_socket_addrs() {
        Ok(a) => {
            a.collect::<Vec<_>>().get(0).cloned()
        }
        Err(_) => {
            s.add(":25565").to_socket_addrs()
                .ok()
                .and_then(|a| a.collect::<Vec<_>>().get(0).cloned())
        }
    };
    match sock_addr {
        Some(addr) => Ok(addr),
        None => Err(serde::de::Error::custom("Invalid socket address")),
    }
}

fn default_bind() -> Vec<SocketAddr> {
    vec!["0.0.0.0:25565".parse().unwrap(), "[::]:25565".parse().unwrap()]
}