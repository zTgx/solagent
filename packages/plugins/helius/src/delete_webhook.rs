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

use solagent_core::SolanaAgentKit;

/// Deletes a Helius Webhook by its ID.
///
/// # Arguments
/// * `agent` - An instance of SolanaAgentKit (with .config.HELIUS_API_KEY)
/// * `webhook_id` - The unique ID of the webhook to delete
///
/// # Returns
/// The response body from the Helius API (which may contain status or other info)
pub async fn delete_webhook(
    agent: &SolanaAgentKit,
    webhook_id: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Get the Helius API key from the agent's configuration
    let api_key = match agent.config.helius_api_key.as_ref() {
        Some(key) => key,
        None => return Err("Missing Helius API key in agent.config.HELIUS_API_KEY".into()),
    };

    // Construct the URL for the DELETE request
    let url = format!("https://api.helius.xyz/v0/webhooks/{}?api-key={}", webhook_id, api_key);

    // Create an HTTP client
    let client = reqwest::Client::new();

    // Send the DELETE request
    let response = client.delete(&url).header("Content-Type", "application/json").send().await?;

    // Check if the request was successful
    if !response.status().is_success() {
        return Err(format!(
            "Failed to delete webhook: {} {}",
            response.status(),
            response.status().canonical_reason().unwrap_or("Unknown")
        )
        .into());
    }

    // Handle different response status codes
    if response.status().as_u16() == 204 {
        return Ok(serde_json::json!({"message": "Webhook deleted successfully (no content returned)"}));
    }

    // Check if the response body is empty
    let content_length = response.headers().get("Content-Length");
    if content_length.is_none() || content_length.expect("HeaderValue").to_str()? == "0" {
        return Ok(serde_json::json!({"message": "Webhook deleted successfully (empty body)"}));
    }

    // Parse the response body as JSON
    let data: serde_json::Value = response.json().await?;
    Ok(data)
}
