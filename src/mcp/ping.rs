use serde::{Deserialize, Serialize};
use crate::configuration::SorryServerConfig;

const DEFAULT_PROTOCOL: u32 = 763;

#[derive(Serialize)]
pub struct Response {
    version: Version,
    players: Players,
    description: Description,
}

impl Response {
    pub fn from_config(config: SorryServerConfig) -> Response {

        let version = Version {
            name: config.version,
            protocol: DEFAULT_PROTOCOL,
        };

        let players = Players {
            max: 0,
            online: 0,
            sample: vec![],
        };

        let description = Description {
            text: config.motd.join("\n"),
        };

        Response {
            version,
            players,
            description,
        }
    }
}

#[derive(Serialize)]
pub struct Version {
    pub name: String,
    pub protocol: u32,
}

#[derive(Serialize)]
pub struct Players {
    pub max: u32,
    pub online: u32,
    pub sample: Vec<Sample>
}

#[derive(Serialize)]
pub struct Sample {
    pub id: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct Description {
    pub text: String,
}

#[cfg(test)]
mod test {
    use super::*;
}