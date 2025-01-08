use crate::{
    actions::{
        deploy_collection, deploy_token, fetch_price, fetch_price_by_pyth,
        fetch_pyth_price_feed_id, get_balance, get_balance_other, get_tps, request_faucet_funds,
        transfer,
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
    ) -> Result<(String, String), ClientError> {
        deploy_collection(&self, &options).await
    }

    pub async fn fetch_price(token_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        fetch_price(token_id).await
    }

    pub async fn fetch_price_by_pyth(
        price_feed_id: &str,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        fetch_price_by_pyth(price_feed_id).await
    }

    pub async fn fetch_pyth_price_feed_id(
        token_symbol: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        fetch_pyth_price_feed_id(token_symbol).await
    }

    pub async fn transfer(
        &self,
        to: &str,
        amount: u64,
        mint: Option<String>,
    ) -> Result<String, ClientError> {
        transfer(&self, to, amount, mint).await
    }
}
