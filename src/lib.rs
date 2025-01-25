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
#![allow(dead_code)]

//! **solagent.rs: Bridging the Gap Between AI and Solana protocols**  
//! solagent.rs is an open-source Rust library designed to streamline the integration of AI agents with Solana protocols. Built upon the rig framework, solagent.rs empowers developers to construct portable, modular, and lightweight full-stack AI agents capable of interacting with the Solana blockchain.
//!
//! This powerful toolkit simplifies agent-to-blockchain communication, offering a comprehensive suite of functions for tasks such as token operations, trading, and more. By leveraging solagent.rs, developers can seamlessly connect their AI agents to the Solana ecosystem, unlocking a world of possibilities for on-chain automation and intelligent decision-making.
//!
//! Quick Start
//!
//! ```rust
//! use std::sync::Arc;
//! use solagent::{Config, SolanaAgentKit, create_solana_tools};
//! #[tokio::main]
//! async fn main() {
//!     let config = Config { openai_api_key: Some("your_api_key".to_string()),
//!             ..Default::default() };
//!     let agent = Arc::new(SolanaAgentKit::new("private_key", "RPC_URL", config));
//!     let toolset = create_solana_tools(agent);
//! }
//! ```
//!
//! Get Balance
//!
//! ```rust
//! use std::sync::Arc;
//! use solagent::{Config, SolanaAgentKit};
//!
//! #[tokio::main]
//! async fn main() {
//!    let config = Config { openai_api_key: Some("your_api_key".to_string()),
//!             ..Default::default() };
//!    let agent = Arc::new(SolanaAgentKit::new("private_key", "RPC_URL", config));
//!    let balance = agent.get_balance(None).await.unwrap();
//!    println!("My balance: {}", balance);
//!}
//! ```
//!
mod actions;
mod agent;
mod primitives;
mod tools;
mod utils;

pub use primitives::token::NFTMetadata;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    bs58,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
pub use tools::*;

pub use ::helius;
pub use anyhow;
pub use async_trait;
pub use base64;
pub use bincode;
pub use lazy_static;
pub use mpl_token_metadata;
pub use reqwest;
pub use rig;
pub use serde;
pub use serde_json;
pub use solana_account_decoder;
pub use solana_client;
pub use solana_program;
pub use solana_sdk;
pub use spl_associated_token_account;
pub use spl_token;
pub use spl_token_2022;
pub use thiserror;

/// Represents the provider for the agent.
/// ref: https://github.com/0xPlaygrounds/rig/tree/main/rig-core/src/providers
// #[non_exhaustive]
// #[derive(Debug, Clone)]
// enum AgentProvider {
//     LOCAL,
//     ANTHROPIC(String),
//     OpenAI(String),
//     Gemini(String),
//     XAI(String),
//     COHERE(String),
//     ETERNALAI(String),
//     PERPLEXITY(String),
// }

/// Wallet
/// - wallet : Wallet keypair for signing transactions
/// - address: Public key of the wallet
struct Wallet {
    pub(crate) wallet: Keypair,
    pub(crate) address: Pubkey,
}

impl Wallet {
    pub fn load(private_key: &str) -> Wallet {
        let secret_key = bs58::decode(private_key).into_vec().expect("private key is not valid base58 format!");
        let wallet = Keypair::from_bytes(&secret_key).expect("Invalid private key!");
        let address = wallet.pubkey();

        Wallet { wallet, address }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub openai_api_key: Option<String>,
    pub jupiter_referral_account: Option<String>,
    pub jupiter_fee_bps: Option<u16>, // Assuming fee is represented as a percentage (0-10000)
    pub flash_privilege: Option<String>,
    pub flexlend_api_key: Option<String>,
    pub helius_api_key: Option<String>,
}

/// Represents a Solana agent that interacts with the blockchain.
/// Provides a unified interface for token operations, NFT management, trading and more
pub struct SolanaAgentKit {
    wallet: Wallet,
    config: Config,
    connection: RpcClient,
}

impl SolanaAgentKit {
    pub fn new(private_key: &str, rpc_url: &str, config: Config) -> Self {
        SolanaAgentKit { wallet: Wallet::load(private_key), config, connection: RpcClient::new(rpc_url) }
    }
}
