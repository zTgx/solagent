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

mod actions;
mod agent;
mod primitives;
mod tools;
mod utils;

pub use primitives::token::NFTMetadata;
pub use tools::*;

use primitives::{config::AgentConfig, wallet::Wallet};
use solana_client::rpc_client::RpcClient;

/// Represents a Solana agent that interacts with the blockchain.
/// Provides a unified interface for token operations, NFT management, trading and more
pub struct SolAgent {
    pub(crate) wallet: Wallet,
    pub(crate) config: AgentConfig,
    pub(crate) connection: RpcClient,
}

impl SolAgent {
    pub fn new(private_key: &str, rpc_url: &str, openai_api_key: &str) -> Self {
        SolAgent {
            wallet: Wallet::load(private_key),
            config: AgentConfig::new(openai_api_key),
            connection: RpcClient::new(rpc_url),
        }
    }
}
