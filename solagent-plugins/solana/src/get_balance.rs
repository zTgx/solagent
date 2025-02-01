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

use solagent_core::{
    solana_client::client_error::ClientError,
    solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey},
    SolanaAgentKit,
};
use std::str::FromStr;

/// Gets the balance of SOL or an SPL token for the agent's wallet.
///
/// # Parameters
///
/// - `agent`: An instance of `SolanaAgentKit`.
/// - `token_address`: An optional SPL token mint address. If not provided, returns the SOL balance.
///
/// # Returns
///
/// A `Result` that resolves to the balance as a number (in UI units) or an error if the account doesn't exist.
pub async fn get_balance(agent: &SolanaAgentKit, token_address: Option<String>) -> Result<f64, ClientError> {
    if let Some(token_address) = token_address {
        // Get SPL token account balance
        if let Ok(pubkey) = Pubkey::from_str(&token_address) {
            let token_account = agent.connection.get_token_account_balance(&pubkey)?;
            let ui_amount = token_account.ui_amount.unwrap_or(0.0);
            return Ok(ui_amount);
        }
    }

    // Get SOL balance
    let balance = agent.connection.get_balance(&agent.wallet.address)?;
    Ok(balance as f64 / LAMPORTS_PER_SOL as f64)
}
