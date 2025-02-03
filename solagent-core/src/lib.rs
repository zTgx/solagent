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

//! **solagent.rs: Bridging the Gap Between AI and Solana protocols**  
//! solagent.rs is an open-source Rust library designed to streamline the integration of AI agents with Solana protocols. Built upon the rig framework, solagent.rs empowers developers to construct portable, modular, and lightweight full-stack AI agents capable of interacting with the Solana blockchain.
//!
//! This powerful toolkit simplifies agent-to-blockchain communication, offering a comprehensive suite of functions for tasks such as token operations, trading, and more. By leveraging solagent.rs, developers can seamlessly connect their AI agents to the Solana ecosystem, unlocking a world of possibilities for on-chain automation and intelligent decision-making.
//!
//!
//! Get TPS
//!
//! ```rust
//! use solagent_core::{Config, SolanaAgentKit};
//!
//! #[tokio::main]
//! async fn main() {
//!    let config = Config { openai_api_key: Some("your_api_key".to_string()),
//!             ..Default::default() };
//!    let agent = SolanaAgentKit::new("private_key", "RPC_URL", config);
//!    let tps = solagent_plugin_solana::get_tps(&agent).await;
//!    println!("tps: {}", tps);
//!}
//! ```
//!

use solagent_wallet_solana::Wallet;
use solana_client::rpc_client::RpcClient;

pub use rig;
pub use serde_json;
pub use solana_client;
pub use solana_program;
pub use solana_sdk;

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub openai_api_key: Option<String>,
    pub jupiter_referral_account: Option<String>,
    pub jupiter_fee_bps: Option<u16>, // Assuming fee is represented as a percentage (0-10000)
    pub flash_privilege: Option<String>,
    pub flexlend_api_key: Option<String>,
    pub helius_api_key: Option<String>,
    pub cookie_api_key: Option<String>,
}

/// Represents a Solana agent that interacts with the blockchain.
/// Provides a unified interface for token operations, NFT management, trading and more
pub struct SolanaAgentKit {
    pub wallet: Wallet,
    pub config: Config,
    pub connection: RpcClient,
}

impl SolanaAgentKit {
    pub fn new(private_key: &str, rpc_url: &str, config: Config) -> Self {
        SolanaAgentKit { wallet: Wallet::load(private_key), config, connection: RpcClient::new(rpc_url) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        bs58,
        signature::{Keypair, Signer},
    };

    #[test]
    fn test_solana_agent_kit_initialization() {
        // Create a new keypair
        let keypair = Keypair::new();
        // Encode the secret key to base58
        let private_key = keypair.to_base58_string();
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let config = Config { openai_api_key: Some("your_api_key".to_string()), ..Default::default() };
        let agent = SolanaAgentKit::new(&private_key, rpc_url, config);
        assert_eq!(
            agent.wallet.address,
            Keypair::from_bytes(&bs58::decode(private_key).into_vec().unwrap()).unwrap().pubkey()
        );
    }
}
