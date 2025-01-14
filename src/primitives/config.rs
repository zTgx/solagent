// Copyright 2025 zTgx
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// Represents the configuration for the agent.
///
/// This struct contains the necessary settings for the agent, including
/// the wallet path, RPC URL, and optional API keys for services.
/// At least one of the API keys must be provided; both cannot be empty.
///
/// Currently supported API keys:
/// - `openai_api_key`: Key for accessing OpenAI services.
/// - `gemini_api_key`: Key for accessing Gemini services.
pub struct AgentConfig {
    /// Optional API key for OpenAI services.
    /// Must be provided if `gemini_api_key` is not set.
    pub openai_api_key: String,
    // pub gemini_api_key: Option<String>,
}

impl AgentConfig {
    pub fn new(openai_api_key: &str) -> Self {
        AgentConfig { openai_api_key: openai_api_key.to_string() }
    }
}
