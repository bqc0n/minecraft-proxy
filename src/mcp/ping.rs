use serde::{Deserialize, Serialize};

struct SorryServerConfig {
    version: String,
    motd: Vec<String>,
    kick_message: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    version: Version,
    players: Players,
    description: Description,
}

#[derive(Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    #[serde(default = "default_protocol")]
    pub protocol: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Players {
    #[serde(default)]
    pub max: u32,
    #[serde(default)]
    pub online: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Description {
    pub text: String,
}

fn default_protocol() -> u32 {
    763
}

#[cfg(test)]
mod test {
    use super::*;
}