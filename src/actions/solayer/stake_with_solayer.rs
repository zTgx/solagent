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

use base64::{engine::general_purpose, Engine};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use solana_sdk::{commitment_config::CommitmentConfig, transaction::VersionedTransaction};

use crate::SolanaAgentKit;

#[derive(Serialize)]
struct StakeRequest {
    account: String,
}

#[derive(Deserialize)]
struct StakeResponse {
    transaction: String,
}

// Stake SOL with Solayer
/// Create a new task on Gibwork
///
/// # Arguments
///
/// * `agent` - SolanaAgentKit instance
/// * `amount` - Amount of SOL to stake
///
/// # Returns
///
/// Transaction signature
pub async fn stake_with_solayer(agent: &SolanaAgentKit, amount: f64) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://app.solayer.org/api/action/restake/ssol?amount={}", amount);
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let request = StakeRequest { account: agent.wallet.address.to_string() };
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .headers(headers)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if !response.status().is_success() {
        let error_data: serde_json::Value =
            response.json().await.map_err(|e| format!("Failed to parse error response: {}", e))?;
        let message = error_data.get("message").and_then(|v| v.as_str()).unwrap_or("Staking request failed");
        return Err(message.to_string().into());
    }

    let stake_response: StakeResponse =
        response.json().await.map_err(|e| format!("Failed to parse stake response: {}", e))?;

    let transaction_data = general_purpose::STANDARD.decode(stake_response.transaction.as_str())?;

    let versioned_transaction: VersionedTransaction = bincode::deserialize(&transaction_data)?;

    let signed_transaction = VersionedTransaction::try_new(versioned_transaction.message, &[&agent.wallet.wallet])?;

    let signature = agent.connection.send_transaction(&signed_transaction)?;

    let latest_blockhash = agent.connection.get_latest_blockhash()?;

    agent.connection.confirm_transaction_with_spinner(&signature, &latest_blockhash, CommitmentConfig::confirmed())?;

    Ok(signature.to_string())
}
