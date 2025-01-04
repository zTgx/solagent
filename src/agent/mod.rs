pub mod agent_impl;

use crate::{
    primitives::config::{AgentConfig, CONFIG},
    primitives::wallet::Wallet,
};
use lazy_static::lazy_static;
use solana_client::rpc_client::RpcClient;

/// Represents a Solana agent that interacts with the blockchain.
pub struct SolAgent {
    pub wallet: Wallet,
    pub config: AgentConfig,
    pub connection: RpcClient,
}

impl SolAgent {
    /// Creates a new instance of `SolAgent` by loading configuration and wallet information.
    pub fn new() -> Self {
        let config = CONFIG.agent.clone();
        let connection = RpcClient::new(config.rpc_url.clone());
        let wallet = Wallet::load(&config.wallet_path);

        SolAgent {
            wallet,
            config,
            connection,
        }
    }
}

lazy_static! {
    /// A static instance of `SolAgent`, initialized lazily.
    /// Main class for interacting with Solana blockchain
    /// Provides a unified interface for token operations, NFT management, trading and more
    pub static ref SOL_AGENT: SolAgent = SolAgent::new();
}
