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
use solana_client::client_error::ClientError;
use solana_sdk::native_token::LAMPORTS_PER_SOL;

/// Requests SOL from the Solana faucet (devnet/testnet only).
///
/// # Parameters
///
/// - `agent`: An instance of `SolanaAgentKit`.
///
/// # Returns
///
/// A transaction signature as a `String`.
///
/// # Errors
///
/// Returns an error if the request fails or times out.
pub async fn request_faucet_funds(agent: &SolanaAgentKit) -> Result<String, ClientError> {
    // Request airdrop of 5 SOL (5 * LAMPORTS_PER_SOL)
    let tx = agent.connection.request_airdrop(&agent.wallet.address, 5 * LAMPORTS_PER_SOL)?;

    // Confirm the transaction
    agent.connection.confirm_transaction(&tx)?;

    Ok(tx.to_string())
}
