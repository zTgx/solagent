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
    fn new() -> Self {
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
    pub static ref SOL_AGENT: SolAgent = SolAgent::new();
}
