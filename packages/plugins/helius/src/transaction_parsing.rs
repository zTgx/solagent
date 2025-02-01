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

use serde::{Deserialize, Serialize};
use serde_json::json;
use solagent_core::SolanaAgentKit;

#[derive(Debug, Serialize, Deserialize)]
pub struct HeliusWebhookIdResponse {
    pub wallet: String,
    pub webhook_url: String,
    pub transaction_types: Vec<String>,
    pub account_addresses: Vec<String>,
    pub webhook_type: String,
}

/// Parse a Solana transaction using the Helius Enhanced Transactions API
///
/// # Arguments
/// * `agent` - An instance of SolanaAgentKit (with .config.HELIUS_API_KEY)
/// * `transaction_id` - The transaction ID to parse
///
/// # Returns
/// Parsed transaction data
pub async fn transaction_parse(
    agent: &SolanaAgentKit,
    transaction_id: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Get the Helius API key from the agent's configuration
    let api_key = match agent.config.helius_api_key.as_ref() {
        Some(key) => key,
        None => return Err("Missing Helius API key in agent.config.HELIUS_API_KEY".into()),
    };

    let client = reqwest::Client::new();
    let url = format!("https://api.helius.xyz/v0/transactions/?api-key={}", api_key);

    let body = json!( {
        "transactions": vec![transaction_id.to_string()],
    });

    let response = client.post(url).header("Content-Type", "application/json").json(&body).send().await?;

    let data = response.json().await?;
    Ok(data)
}
