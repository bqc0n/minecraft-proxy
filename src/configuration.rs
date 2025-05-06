use serde::Deserialize;
use std::collections::HashMap;
use std::net::{SocketAddr, ToSocketAddrs};
use std::ops::Add;
use std::time::Duration;

#[derive(Deserialize, Clone)]
pub(crate) struct Configuration {
    #[serde(rename = "servers")]
    pub proxies: HashMap<String, ProxyConfig>,
    pub sorry_server: Option<SorryServerConfig>,
    #[serde(default)]
    pub health_check: HealthCheck,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProxyConfig {
    #[serde(default = "default_bind")]
    pub bind: Vec<SocketAddr>,
    #[serde(deserialize_with = "deserialize_server")]
    pub server: SocketAddr,
    #[serde(default)] // Default to false
    pub proxy_protocol: bool,
}

#[derive(Deserialize, Clone)]
pub(crate) struct SorryServerConfig {
    pub(crate) version: String,
    pub(crate) motd: Vec<String>,
    pub(crate) kick_message: Vec<String>,
}

#[derive(Deserialize, Clone)]
#[serde(default)]
pub(crate) struct HealthCheck {
    pub(crate) enabled: bool,
    pub(crate) interval_sec: u64,
    pub(crate) timeout_sec: u64,
    pub(crate) max_failures: u8,
}

impl Default for HealthCheck {
    fn default() -> Self {
        Self {
            enabled: false,
            interval_sec: 5,
            timeout_sec: 2,
            max_failures: 3,
        }
    }
}

impl HealthCheck {
    pub fn interval(&self) -> Duration {
        Duration::from_secs(self.interval_sec)
    }

    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_sec)
    }
}

fn deserialize_server<'de, D>(deserializer: D) -> Result<SocketAddr, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let sock_addr = match s.to_socket_addrs() {
        Ok(a) => a.collect::<Vec<_>>().get(0).cloned(),
        Err(_) => s
            .add(":25565")
            .to_socket_addrs()
            .ok()
            .and_then(|a| a.collect::<Vec<_>>().get(0).cloned()),
    };
    match sock_addr {
        Some(addr) => Ok(addr),
        None => Err(serde::de::Error::custom("Invalid socket address")),
    }
}

fn default_bind() -> Vec<SocketAddr> {
    vec![
        "0.0.0.0:25565".parse().unwrap(),
        "[::]:25565".parse().unwrap(),
    ]
}
