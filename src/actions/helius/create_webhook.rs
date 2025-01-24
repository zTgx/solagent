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

use crate::SolanaAgentKit;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct HeliusWebhookResponse {
    pub webhook_url: String,
    pub webhook_id: String,
}

pub async fn create_webhook(
    agent: &SolanaAgentKit,
    account_addresses: Vec<String>,
    webhook_url: String,
) -> Result<HeliusWebhookResponse, Box<dyn std::error::Error>> {
    if agent.config.helius_api_key.is_none() {
        return Err("helius api key is none.".into());
    }

    let api_key = &agent.config.helius_api_key.clone().unwrap();
    let url = format!("https://api.helius.xyz/v0/webhooks?api-key={}", api_key);

    let body = serde_json::json!({
        "webhookURL": webhook_url,
        "transactionTypes": ["Any"],
        "accountAddresses": account_addresses,
        "webhookType": "enhanced",
        "txnStatus": "all",
    });

    let client = reqwest::Client::new();
    let response = client.post(url).header("Content-Type", "application/json").json(&body).send().await?;

    let data = response.json::<serde_json::Value>().await?;
    let webhook_url = data.get("webhookURL").expect("webhookURL field").as_str().expect("webhookURL text");
    let webhook_id = data.get("webhookID").expect("webhookID field").as_str().expect("webhookID text");

    Ok(HeliusWebhookResponse { webhook_url: webhook_url.to_string(), webhook_id: webhook_id.to_string() })
}
