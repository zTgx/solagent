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

mod config;

pub use config::{Config, ConfigBuilder};
pub use rig;
pub use serde_json;
pub use solana_client;
pub use solana_program;
pub use solana_sdk;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

pub trait IWallet {
    fn pubkey(&self) -> Pubkey;
    fn keypair(&self) -> &Keypair;
    fn to_base58(&self) -> String;
}

/// Represents a Solana agent that interacts with the blockchain.
/// Provides a unified interface for token operations, NFT management, trading and more
pub struct SolanaAgentKit<W: IWallet> {
    pub wallet: W,
    pub config: Config,
    pub connection: RpcClient,
}

impl<W: IWallet> SolanaAgentKit<W> {
    pub fn new(wallet: W, rpc_url: &str, config: Config) -> Self {
        let connection = RpcClient::new(rpc_url);
        Self { wallet, config, connection }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::signature::{Keypair, Signer};

    fn mock_wallet() -> impl IWallet {
        pub struct MyWallet {
            pub keypair: Keypair,
            pub pubkey: Pubkey,
        }

        impl IWallet for MyWallet {
            // Implement the trait
            fn pubkey(&self) -> Pubkey {
                self.pubkey
            }
            fn keypair(&self) -> &Keypair {
                &self.keypair
            }
            fn to_base58(&self) -> String {
                self.keypair.to_base58_string()
            }
        }

        impl MyWallet {
            pub fn new() -> Self {
                let keypair = Keypair::new();
                let pubkey = keypair.pubkey();
                Self { keypair, pubkey }
            }
        }

        MyWallet::new()
    }

    #[test]
    fn test_solana_agent_kit_initialization() {
        let wallet = mock_wallet();
        let wallet_pubkey = wallet.pubkey();
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let config = Config { openai_api_key: Some("your_api_key".to_string()), ..Default::default() };
        let agent = SolanaAgentKit::new(wallet, rpc_url, config);

        assert_eq!(agent.wallet.pubkey(), wallet_pubkey);
    }

    #[test]
    fn test_solana_agent_kit_with_config_builder() {
        let wallet = mock_wallet();
        let wallet_pubkey = wallet.pubkey();

        let rpc_url = "https://api.mainnet-beta.solana.com";
        let config = ConfigBuilder::default().openai_api_key("test_api_key".to_string()).jupiter_fee_bps(500).build();

        let agent = SolanaAgentKit::new(wallet, rpc_url, config);

        assert_eq!(agent.config.openai_api_key, Some("test_api_key".to_string()));
        assert_eq!(agent.config.jupiter_fee_bps, Some(500));
        assert_eq!(agent.wallet.pubkey(), wallet_pubkey);
    }
}
