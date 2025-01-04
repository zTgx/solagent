use crate::{
    actions::{
        deploy_collection, deploy_token, get_balance, get_balance_other, get_tps,
        request_faucet_funds,
    },
    agent::SolAgent,
    primitives::token::CollectionOptions,
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

    pub async fn get_tps(&self) -> Result<f64, ClientError> {
        get_tps(&self).await
    }

    pub async fn deploy_token(
        &self,
        name: String,
        uri: String,
        symbol: String,
        decimals: u8,
        initial_supply: Option<u64>,
    ) -> Result<Pubkey, ClientError> {
        deploy_token(&self, name, uri, symbol, decimals, initial_supply).await
    }

    pub async fn deploy_collection(
        &self,
        options: CollectionOptions,
    ) -> Result<Pubkey, ClientError> {
        deploy_collection(&self, &options).await
    }
}
