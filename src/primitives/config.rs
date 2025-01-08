use serde::Deserialize;

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
    /// Optional API key for OpenAI services.
    /// Must be provided if `gemini_api_key` is not set.
    pub openai_api_key: String,
    // pub gemini_api_key: Option<String>,
}

impl AgentConfig {
    pub fn new(openai_api_key: &str) -> Self {
        AgentConfig {
            openai_api_key: openai_api_key.to_string(),
        }
    }
}
