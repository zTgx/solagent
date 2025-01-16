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

use crate::{primitives::constants::JUP_API, SolanaAgentKit};
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    commitment_config::CommitmentConfig, program_pack::Pack, pubkey::Pubkey, transaction::VersionedTransaction,
};
use spl_token::state::Mint;
use std::str::FromStr;

#[derive(Serialize)]
struct SwapRequest {
    #[serde(rename = "quoteResponse")]
    quote_response: QuoteResponse,
    #[serde(rename = "userPublicKey")]
    user_public_key: String,
    #[serde(rename = "wrapAndUnwrapSol")]
    wrap_and_unwrap_sol: bool,
    #[serde(rename = "dynamicComputeUnitLimit")]
    dynamic_compute_unit_limit: bool,
    #[serde(rename = "prioritizationFeeLamports")]
    prioritization_fee_lamports: String,
    #[serde(rename = "feeAccount")]
    fee_account: Option<String>,
}
#[derive(Deserialize)]
struct SwapResponse {
    #[serde(rename = "swapTransaction")]
    swap_transaction: String,
}

#[derive(Deserialize, Serialize)]
struct QuoteResponse {
    #[serde(flatten)]
    extra: serde_json::Value,
}

/// Swap tokens using Jupiter Exchange
///
/// # Arguments
///
/// * `agent` - SolanaAgentKit instance
/// * `output_mint` - Target token mint address
/// * `input_amount` - Amount to swap (in token decimals)
/// * `input_mint` - Source token mint address (defaults to SOL)
/// * `slippage_bps` - Slippage tolerance in basis points (default: 300 = 3%)
///
/// # Returns
///
/// Transaction signature as a string
pub async fn trade(
    agent: &SolanaAgentKit,
    output_mint: &str,
    input_amount: f64,
    input_mint: Option<String>,
    slippage_bps: Option<u32>,
) -> Result<String, Box<dyn std::error::Error>> {
    // Convert strings to Pubkeys
    let output_mint = Pubkey::from_str(output_mint)?;
    let input_mint = input_mint.as_deref().map(Pubkey::from_str).transpose()?.unwrap_or(spl_token::native_mint::id());

    // Use defaults if not provided
    let slippage_bps = slippage_bps.unwrap_or(300);

    // Check if input token is native SOL
    let is_native_sol = input_mint == spl_token::native_mint::id();

    // Get input token decimals
    let input_decimals = if is_native_sol {
        9
    } else {
        let account = agent.connection.get_account(&input_mint)?;
        let mint = Mint::unpack(&account.data)?;
        mint.decimals
    };

    // Calculate scaled amount
    let scaled_amount = (input_amount * 10f64.powf(input_decimals as f64)) as u64;

    // Build quote URL
    let quote_url = format!(
        "{}/quote?inputMint={}&outputMint={}&amount={}&slippageBps={}&onlyDirectRoutes=true&maxAccounts=20",
        JUP_API, input_mint, output_mint, scaled_amount, slippage_bps
    );

    // Get quote
    let client = reqwest::Client::new();
    let quote_response: QuoteResponse = client.get(&quote_url).send().await?.json().await?;

    // Get swap transaction
    let swap_request = SwapRequest {
        quote_response,
        user_public_key: agent.wallet.address.to_string(),
        wrap_and_unwrap_sol: true,
        dynamic_compute_unit_limit: true,
        prioritization_fee_lamports: "auto".to_string(),
        fee_account: None,
    };

    let swap_response: SwapResponse =
        client.post(format!("{}/swap", JUP_API)).json(&swap_request).send().await?.json().await?;

    let swap_transaction = general_purpose::STANDARD.decode(&swap_response.swap_transaction).unwrap();

    let versioned_transaction: VersionedTransaction = bincode::deserialize(&swap_transaction)?;

    let signed_transaction = VersionedTransaction::try_new(versioned_transaction.message, &[&agent.wallet.wallet])?;

    let signature = agent.connection.send_transaction(&signed_transaction)?;

    let latest_blockhash = agent.connection.get_latest_blockhash()?;

    agent.connection.confirm_transaction_with_spinner(&signature, &latest_blockhash, CommitmentConfig::confirmed())?;

    Ok(signature.to_string())
}
