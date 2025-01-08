mod agent_impl;

use crate::{primitives::config::AgentConfig, primitives::wallet::Wallet};
use solana_client::rpc_client::RpcClient;

/// Represents a Solana agent that interacts with the blockchain.
/// Main class for interacting with Solana blockchain
/// Provides a unified interface for token operations, NFT management, trading and more
pub struct SolAgent {
    pub wallet: Wallet,
    pub config: AgentConfig,
    pub connection: RpcClient,
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
