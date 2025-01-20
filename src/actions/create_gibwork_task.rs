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
use serde::{Deserialize, Serialize};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, transaction::VersionedTransaction};

#[derive(Serialize)]
struct TaskRequest {
    title: String,
    content: String,
    requirements: String,
    tags: Vec<String>,
    payer: String,
    token: TokenInfo,
}

#[derive(Serialize)]
struct TokenInfo {
    #[serde(rename = "mintAddress")]
    mint_address: String,
    amount: u64,
}

#[derive(Deserialize)]
struct TaskResponse {
    #[serde(rename = "taskId")]
    task_id: String,
    #[serde(rename = "serializedTransaction")]
    serialized_transaction: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GibworkCreateTaskResponse {
    pub status: String,
    pub task_id: String,
    pub signature: String,
}

/// Create a new task on Gibwork
///
/// # Arguments
///
/// * `agent` - SolanaAgentKit instance
/// * `title` - Title of the task
/// * `content` - Description of the task
/// * `requirements` - Requirements to complete the task
/// * `tags` - List of tags associated with the task
/// * `token_mint_address` - Token mint address for payment
/// * `token_amount` - Payment amount for the task
/// * `payer` - Optional payer address (defaults to agent's wallet address)
///
/// # Returns
///
/// Object containing task creation transaction and generated taskId
#[allow(clippy::too_many_arguments)]
pub async fn create_gibwork_task(
    agent: &SolanaAgentKit,
    title: &str,
    content: &str,
    requirements: &str,
    tags: Vec<String>,
    token_mint_address: &str,
    token_amount: u64,
    payer: Option<Pubkey>,
) -> Result<GibworkCreateTaskResponse, Box<dyn std::error::Error>> {
    let request = TaskRequest {
        title: title.to_string(),
        content: content.to_string(),
        requirements: requirements.to_string(),
        tags,
        payer: payer.unwrap_or(agent.wallet.address).to_string(),
        token: TokenInfo { mint_address: token_mint_address.to_string(), amount: token_amount },
    };

    // Send request to Gibwork API
    let client = reqwest::Client::new();
    let response = client.post("https://api2.gib.work/tasks/public/transaction").json(&request).send().await?;

    if !response.status().is_success() {
        return Err(format!("API request failed: {}", response.status()).into());
    }

    let task_response: TaskResponse = response.json().await?;

    // Deserialize and sign transaction
    let transaction_data = general_purpose::STANDARD.decode(task_response.serialized_transaction.as_str())?;

    let mut versioned_transaction: VersionedTransaction = bincode::deserialize(&transaction_data)?;

    // Get latest blockhash and sign transaction
    let blockhash = agent.connection.get_latest_blockhash()?;
    versioned_transaction.message.set_recent_blockhash(blockhash);
    let signed_transaction = VersionedTransaction::try_new(versioned_transaction.message, &[&agent.wallet.wallet])?;

    // Send and confirm transaction
    let signature = agent.connection.send_transaction(&signed_transaction)?;
    agent.connection.confirm_transaction_with_spinner(&signature, &blockhash, CommitmentConfig::confirmed())?;

    Ok(GibworkCreateTaskResponse {
        status: "success".to_string(),
        task_id: task_response.task_id,
        signature: signature.to_string(),
    })
}
