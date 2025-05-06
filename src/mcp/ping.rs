use crate::configuration::SorryServerConfig;
use crate::mcp::constants::DEFAULT_PROTOCOL;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Version {
    pub name: String,
    pub protocol: u32,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Players {
    pub max: u32,
    pub online: u32,
    pub sample: Vec<Sample>
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Sample {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Description {
    pub text: String,
}