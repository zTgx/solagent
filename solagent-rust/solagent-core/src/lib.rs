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

mod config;

use config::Config;
pub use config::ConfigBuilder;

pub use rig;
pub use solana_client;
pub use solana_program;
pub use solana_sdk;

use solagent_wallet_solana::Wallet;

/// Represents a Solana agent that interacts with the blockchain.
/// Provides a unified interface for token operations, NFT management, trading and more
pub struct SolanaAgentKit {
    pub wallet: Wallet,
    pub config: Config,
    pub connection: solana_client::rpc_client::RpcClient,
}

impl SolanaAgentKit {
    pub fn new(wallet: Wallet, rpc_url: &str, config: Config) -> Self {
        let connection = solana_client::rpc_client::RpcClient::new(rpc_url);
        Self { wallet, config, connection }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solana_agent_kit_initialization() {
        let wallet = Wallet::new();
        let wallet_pubkey = wallet.pubkey;
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let config = Config { openai_api_key: Some("your_api_key".to_string()), ..Default::default() };
        let agent = SolanaAgentKit::new(wallet, rpc_url, config);

        assert_eq!(agent.wallet.pubkey, wallet_pubkey);
    }

    #[test]
    fn test_solana_agent_kit_with_config_builder() {
        let wallet = Wallet::new();
        let wallet_pubkey = wallet.pubkey;

        let rpc_url = "https://api.mainnet-beta.solana.com";
        let config = ConfigBuilder::default().openai_api_key("test_api_key".to_string()).jupiter_fee_bps(500).build();

        let agent = SolanaAgentKit::new(wallet, rpc_url, config);

        assert_eq!(agent.config.openai_api_key, Some("test_api_key".to_string()));
        assert_eq!(agent.config.jupiter_fee_bps, Some(500));
        assert_eq!(agent.wallet.pubkey, wallet_pubkey);
    }
}
