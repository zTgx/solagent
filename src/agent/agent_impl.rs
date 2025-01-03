use crate::{
    actions::{get_balance, get_balance_other, request_faucet_funds},
    agent::SolAgent,
};
use solana_client::client_error::ClientError;
use solana_sdk::pubkey::Pubkey;

impl SolAgent {
    pub async fn get_balance(&self, token_address: Option<String>) -> Result<f64, ClientError> {
        get_balance(&self, token_address).await
    }

    pub async fn request_faucet_funds(&self) -> Result<String, ClientError> {
        request_faucet_funds(&self).await
    }

    pub async fn get_balance_other(
        &self,
        wallet_address: Pubkey,
        token_address: Option<Pubkey>,
    ) -> Result<f64, ClientError> {
        get_balance_other(&self, wallet_address, token_address).await
    }
}
