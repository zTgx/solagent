use lazy_static::lazy_static;
use serde::Deserialize;
use std::{fs, path::PathBuf};

/// Represents the configuration for the agent.
///
/// This struct contains the necessary settings for the agent, including
/// the wallet path, RPC URL, and optional API keys for services.
/// At least one of the API keys must be provided; both cannot be empty.
///
/// Currently supported API keys:
/// - `openai_api_key`: Key for accessing OpenAI services.
/// - `gemini_api_key`: Key for accessing Gemini services.
#[derive(Deserialize, Debug, Clone)]
pub struct AgentConfig {
    /// Path to the wallet file.
    pub wallet_path: String,

    /// URL of the RPC endpoint for Solana.
    pub rpc_url: String,

    /// Optional API key for OpenAI services.
    /// Must be provided if `gemini_api_key` is not set.
    pub openai_api_key: Option<String>,

    /// Optional API key for Gemini services.
    /// Must be provided if `openai_api_key` is not set.
    pub gemini_api_key: Option<String>,
}

/// Represents the configuration structure for the application.
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    /// Configuration for the agent.
    pub agent: AgentConfig,
}

lazy_static! {
    /// A static instance of the application configuration, loaded from a TOML file.
    ///
    /// This uses `lazy_static` to ensure that the configuration is only loaded once
    /// and is accessible throughout the application.
    pub static ref CONFIG: Config = load_config("config.toml").expect("Failed to load config.toml");
}

/// Loads the configuration from a specified TOML file.
///
/// # Parameters
///
/// - `filename`: The name of the configuration file (e.g., "config.toml").
///
/// # Returns
///
/// Returns a `Result` containing the `Config` if successful, or an error if loading fails.
fn load_config(filename: &str) -> Result<Config, Box<dyn std::error::Error>> {
    // Build the path to the configuration file based on the current directory.
    let mut path = PathBuf::from(std::env::current_dir()?);
    path.push(filename);

    // Read the contents of the configuration file into a string.
    let contents = fs::read_to_string(path)?;

    // Parse the TOML string into a Config struct.
    let config: Config = toml::from_str(&contents)?;

    Ok(config)
}
