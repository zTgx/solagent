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

use serde_json::json;
use solagent_core::SolanaAgentKit;

pub async fn get_assets_by_owner(
    agent: &SolanaAgentKit,
    owner_public_key: &str,
    limit: u32,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Get the Helius API key from the agent's configuration
    let api_key = match agent.config.helius_api_key.as_ref() {
        Some(key) => key,
        None => return Err("Missing Helius API key in agent.config.HELIUS_API_KEY".into()),
    };

    let url = format!("https://mainnet.helius-rpc.com/?api-key={}", api_key);

    let client = reqwest::Client::new();

    let request_body = json!({
        "jsonrpc": "2.0",
        "id": "get-assets",
        "method": "getAssetsByOwner",
        "params": json!({
            "ownerAddress": owner_public_key,
            "page": 3,
            "limit": limit,
            "displayOptions": { "showFungible": true },
        }),
    });

    let response = client.post(&url).header("Content-Type", "application/json").json(&request_body).send().await?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to fetch: {} - {}",
            response.status(),
            response.status().canonical_reason().unwrap_or("Unknown")
        )
        .into());
    }

    let data: serde_json::Value = response.json().await?;

    Ok(data)
}
