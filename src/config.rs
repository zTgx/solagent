use lazy_static::lazy_static;
use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Deserialize, Debug, Clone)]
pub struct AgentConfig {
    pub wallet_path: String,
    pub rpc_url: String,
    pub api_key: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub agent: AgentConfig,
}

lazy_static! {
    pub static ref CONFIG: Config = load_config("config.toml").expect("Failed to load config.toml");
}

fn load_config(filename: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut path = PathBuf::from(std::env::current_dir()?);
    path.push(filename);

    let contents = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}
