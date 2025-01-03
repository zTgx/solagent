use crate::{agent::SolAgent, tools::get_balance};
use solana_client::client_error::ClientError;
use solana_sdk::pubkey::Pubkey;

impl SolAgent {
    pub async fn get_balance(&self, token_address: Option<Pubkey>) -> Result<f64, ClientError> {
        get_balance::call(&self, token_address).await
    }
}
