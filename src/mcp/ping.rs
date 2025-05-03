use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    version: Version,
    players: Players,
    description: Description,
}

#[derive(Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    pub protocol: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Players {
    pub max: u32,
    pub online: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Description {
    pub text: String,
}

#[cfg(test)]
mod test {
    use super::*;
}