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
use solagent_core::SolanaAgentKit;

#[derive(Debug, Serialize, Deserialize)]
pub struct HeliusWebhookIdResponse {
    pub wallet: String,
    pub webhook_url: String,
    pub transaction_types: Vec<String>,
    pub account_addresses: Vec<String>,
    pub webhook_type: String,
}

/// Retrieves a Helius Webhook by ID, returning only the specified fields.
///
/// # Arguments
/// * `agent` - An instance of SolanaAgentKit (with .config.HELIUS_API_KEY)
/// * `webhook_id` - The unique ID of the webhook to delete
///
/// # Returns
/// A HeliusWebhook object containing { wallet, webhookURL, transactionTypes, accountAddresses, webhookType }
pub async fn get_webhook(
    agent: &SolanaAgentKit,
    webhook_id: &str,
) -> Result<HeliusWebhookIdResponse, Box<dyn std::error::Error>> {
    // Get the Helius API key from the agent's configuration
    let api_key = match agent.config.helius_api_key.as_ref() {
        Some(key) => key,
        None => return Err("Missing Helius API key in agent.config.HELIUS_API_KEY".into()),
    };

    let client = reqwest::Client::new();
    let url = format!("https://api.helius.xyz/v0/webhooks/{}?api-key={}", webhook_id, api_key);

    let response = client.get(url).header("Content-Type", "application/json").send().await?;

    let data = response.json::<HeliusWebhookIdResponse>().await?;
    Ok(data)
}
