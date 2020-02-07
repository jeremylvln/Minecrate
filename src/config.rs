use std::io;
use std::fs::File;
use std::io::Read;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub motd: String,
    pub max_players: u32,
}

impl Config {
    pub fn from_path(path: &str) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Ok(toml::from_str(&content)?)
    }

    pub fn default() -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port: 25565,
            motd: String::from("Minecrate server"),
            max_players: 20,
        }
    }
}