pub mod agent_impl;

use crate::{
    config::{AgentConfig, CONFIG},
    primitives::wallet::Wallet,
};
use solana_client::nonblocking::rpc_client::RpcClient;

pub struct SolAgent {
    pub wallet: Wallet,
    pub config: AgentConfig,
    pub connection: RpcClient,
}

impl SolAgent {
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
