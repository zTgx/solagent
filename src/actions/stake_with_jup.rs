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
use base64::{engine::general_purpose, Engine};
use solana_sdk::{commitment_config::CommitmentConfig, transaction::VersionedTransaction};

/// Stake SOL with Jupiter validator
///
/// # Arguments
///
/// * `agent` - SolanaAgentKit instance
/// * `amount` - Amount of SOL to stake (in SOL)
///
/// # Returns
///
/// Transaction signature as a string
pub async fn stake_with_jup(agent: &SolanaAgentKit, amount: f64) -> Result<String, Box<dyn std::error::Error>> {
    // Convert SOL amount to lamports
    let amount_lamports = (amount * 1e9) as u64;

    // Build stake URL
    let stake_url = format!(
        "https://worker.jup.ag/blinks/swap/So11111111111111111111111111111111111111112/jupSoLaHXQiZZTSfEWMTRRgpnyFm8f6sZdosWBjx93v/{}",
        amount_lamports
    );

    // Get stake transaction
    let client = reqwest::Client::new();
    let stake_request = serde_json::json!({
        "account": agent.wallet.address.to_string(),
    });

    let response = client.post(&stake_url).json(&stake_request).send().await?;

    let data: serde_json::Value = response.json().await?;
    let transaction_data = general_purpose::STANDARD.decode(data["transaction"].as_str().unwrap())?;

    let mut versioned_transaction: VersionedTransaction = bincode::deserialize(&transaction_data)?;

    let blockhash = agent.connection.get_latest_blockhash()?;
    versioned_transaction.message.set_recent_blockhash(blockhash);

    // Sign and send transaction
    let signed_transaction = VersionedTransaction::try_new(versioned_transaction.message, &[&agent.wallet.wallet])?;

    let signature = agent.connection.send_transaction(&signed_transaction)?;

    // Confirm transaction
    let latest_blockhash = agent.connection.get_latest_blockhash()?;
    agent.connection.confirm_transaction_with_spinner(&signature, &latest_blockhash, CommitmentConfig::confirmed())?;

    Ok(signature.to_string())
}
