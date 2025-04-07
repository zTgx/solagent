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

use solagent_core::{solana_sdk::signature::Keypair, Config, SolanaAgentKit};
use solagent_plugin_jupiter::{stake_with_jup, trade};

/// Example on devnet
/// Mint: 5jcsea3EA3kX7mXpy7YvHVFYTDEJeSEXjyicgThnvWUm
/// https://explorer.solana.com/address/5jcsea3EA3kX7mXpy7YvHVFYTDEJeSEXjyicgThnvWUm?cluster=devnet

#[tokio::main]
async fn main() {
    // Create a new keypair
    let keypair = Keypair::new();
    // Encode the secret key to base58
    let private_key = keypair.to_base58_string();

    let config = Config { cookie_api_key: Some("".to_string()), ..Default::default() };
    let agent = SolanaAgentKit::new(&private_key, "https://api.devnet.solana.com", config);

    //swap 0.01 SOL to USDC
    let swap = trade(
        &agent,
        "So11111111111111111111111111111111111111112",
        0.01,
        Some("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string()),
        None,
    )
    .await
    .unwrap();
    println!("Signature: {}", swap);

    //stake 0.01 SOL
    let stake = stake_with_jup(&agent, 0.01).await.unwrap();
    println!("Signature: {}", stake);
}
