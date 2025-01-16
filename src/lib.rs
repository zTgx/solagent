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

//! `solagent.rs`(Solana Agent) is an open-source Rust library that simplifies connecting any AI agent to Solana protocols.
//! Built on the rig framework, which empowers developers to create portable, modular, and lightweight full-stack AI agents.
//!
//! Quick Start
//!
//! ```rust
//! use std::sync::Arc;
//! use solagent::{AgentProvider, SolanaAgentKit, create_solana_tools};
//! #[tokio::main]
//! async fn main() {
//!     let agent = Arc::new(SolanaAgentKit::new("private_key_bs58",
//!                                         "rpc_url",
//!                          AgentProvider::OpenAI("key".into())));
//!     let toolset = create_solana_tools(agent);
//! }
//! ```
//!
//! Get Balance
//!
//! ```rust
//! use std::sync::Arc;
//! use solagent::{AgentProvider, SolanaAgentKit};
//!
//! #[tokio::main]
//! async fn main() {
//!     let agent = Arc::new(SolanaAgentKit::new("private_key_bs58",
//!                                         "rpc_url",
//!                          AgentProvider::OpenAI("key".into())));
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

/// Represents the provider for the agent.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum AgentProvider {
    Local,
    ANTHROPIC(String),
    OpenAI(String),
    Gemini(String),
    XAI(String),
    COHERE(String),
    ETERNALAI(String),
    PERPLEXITY(String),
}

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

/// Represents a Solana agent that interacts with the blockchain.
/// Provides a unified interface for token operations, NFT management, trading and more
pub struct SolanaAgentKit {
    wallet: Wallet,
    provider: AgentProvider,
    connection: RpcClient,
}

impl SolanaAgentKit {
    pub fn new(private_key: &str, rpc_url: &str, provider: AgentProvider) -> Self {
        SolanaAgentKit { wallet: Wallet::load(private_key), provider, connection: RpcClient::new(rpc_url) }
    }
}
