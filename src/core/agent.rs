use crate::{
    config::{AgentConfig, CONFIG},
    primitives::wallet::Wallet, tools::get_balance,
};
use solana_client::{client_error::ClientError, nonblocking::rpc_client::RpcClient};
use solana_sdk::pubkey::Pubkey;

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

impl SolAgent {
    pub async fn get_balance(&self, token_address: Option<Pubkey>) -> Result<f64, ClientError> {
        get_balance::call(&self, token_address).await
    }
}
