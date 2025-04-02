use serde::Deserialize;
use std::fs::read_to_string;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub directories: Directories,
    pub theme: Theme,
    pub server: Server,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub domain: String,
    pub port: usize,
}

#[derive(Debug, Deserialize)]
pub struct Directories {
    pub music: String,
    pub output: String,
    pub templates: String,
}

#[derive(Debug, Deserialize)]
pub struct Theme {
    pub title: String,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_raw =
        read_to_string("config.toml").expect("No config.toml found in working directory.");
    let config: Config = toml::from_str(&config_raw)?;
    Ok(config)
}
