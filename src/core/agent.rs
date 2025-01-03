use crate::{
    config::{AgentConfig, CONFIG},
    primitives::wallet::Wallet,
};
use solana_client::{client_error::ClientError, nonblocking::rpc_client::RpcClient};
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};

pub struct SolAgent {
    wallet: Wallet,
    pub config: AgentConfig,
    connection: RpcClient,
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
        if token_address.is_none() {
            // Get SOL balance
            let balance = self.connection.get_balance(&self.wallet.address).await?;
            return Ok(balance as f64 / LAMPORTS_PER_SOL as f64);
        }

        // Get SPL token account balance
        let token_account = self
            .connection
            .get_token_account_balance(&token_address.unwrap())
            .await?;
        let ui_amount = token_account.ui_amount.unwrap_or(0.0);
        Ok(ui_amount)
    }
}
