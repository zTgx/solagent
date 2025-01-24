// Copyright 2025 zTgx
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
    actions::{
        close_empty_token_accounts, create_gibwork_task, create_webhook, deploy_collection, deploy_token,
        fetch_detailed_report, fetch_price, fetch_price_by_pyth, fetch_pyth_price_feed_id, fetch_summary_report,
        get_balance, get_balance_other, get_token_data_by_address, get_tps, launch_token_pumpfun,
        mint_nft_to_collection, request_faucet_funds, stake_with_jup, stake_with_solayer, trade, transfer,
        GibworkCreateTaskResponse, HeliusWebhookResponse,
    },
    primitives::{
        close_empty_token_accounts::CloseEmptyTokenAccountsData,
        pumpfun::{PumpFunTokenOptions, PumpfunTokenResponse},
        rugcheck::TokenCheck,
        token::{DeployedData, NFTMetadata},
    },
    SolanaAgentKit,
};
use solana_client::client_error::ClientError;
use solana_sdk::pubkey::Pubkey;

impl SolanaAgentKit {
    pub async fn get_balance(&self, token_address: Option<String>) -> Result<f64, ClientError> {
        get_balance(self, token_address).await
    }

    pub async fn request_faucet_funds(&self) -> Result<String, ClientError> {
        request_faucet_funds(self).await
    }

    pub async fn get_balance_other(
        &self,
        wallet_address: Pubkey,
        token_address: Option<Pubkey>,
    ) -> Result<f64, ClientError> {
        get_balance_other(self, wallet_address, token_address).await
    }

    pub async fn get_tps(&self) -> Result<f64, ClientError> {
        get_tps(self).await
    }

    pub async fn deploy_token(
        &self,
        name: String,
        uri: String,
        symbol: String,
        decimals: u8,
        initial_supply: Option<u64>,
    ) -> Result<DeployedData, ClientError> {
        deploy_token(self, name, uri, symbol, decimals, initial_supply).await
    }

    pub async fn deploy_collection(&self, metadata: NFTMetadata) -> Result<DeployedData, ClientError> {
        deploy_collection(self, &metadata).await
    }

    pub async fn mint_nft_to_collection(
        &self,
        collection: Pubkey,
        metadata: NFTMetadata,
    ) -> Result<DeployedData, ClientError> {
        mint_nft_to_collection(self, collection, metadata).await
    }

    pub async fn fetch_price(token_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        fetch_price(token_id).await
    }

    pub async fn fetch_price_by_pyth(price_feed_id: &str) -> Result<f64, Box<dyn std::error::Error>> {
        fetch_price_by_pyth(price_feed_id).await
    }

    pub async fn fetch_pyth_price_feed_id(token_symbol: &str) -> Result<String, Box<dyn std::error::Error>> {
        fetch_pyth_price_feed_id(token_symbol).await
    }

    pub async fn transfer(&self, to: &str, amount: u64, mint: Option<String>) -> Result<String, ClientError> {
        transfer(self, to, amount, mint).await
    }

    pub async fn launch_token_pumpfun(
        &self,
        token_name: &str,
        token_ticker: &str,
        description: &str,
        image_url: &str,
        options: Option<PumpFunTokenOptions>,
    ) -> Result<PumpfunTokenResponse, Box<dyn std::error::Error>> {
        launch_token_pumpfun(self, token_name, token_ticker, description, image_url, options).await
    }

    pub async fn trade(
        &self,
        from_token: Option<String>,
        amount: f64,
        to_token: &str,
        slippage: Option<u32>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        trade(self, to_token, amount, from_token, slippage).await
    }

    pub async fn stake_with_jup(&self, amount: f64) -> Result<String, Box<dyn std::error::Error>> {
        stake_with_jup(self, amount).await
    }

    pub async fn fetch_summary_report(&self, mint: String) -> Result<TokenCheck, Box<dyn std::error::Error>> {
        fetch_summary_report(mint).await
    }

    pub async fn fetch_detailed_report(mint: String) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        fetch_detailed_report(mint).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create_gibwork_task(
        &self,
        title: &str,
        content: &str,
        requirements: &str,
        tags: Vec<String>,
        token_mint_address: &str,
        token_amount: u64,
        payer: Option<Pubkey>,
    ) -> Result<GibworkCreateTaskResponse, Box<dyn std::error::Error>> {
        create_gibwork_task(self, title, content, requirements, tags, token_mint_address, token_amount, payer).await
    }

    pub async fn close_empty_token_accounts(&self) -> Result<CloseEmptyTokenAccountsData, Box<dyn std::error::Error>> {
        close_empty_token_accounts(self).await
    }

    pub async fn stake_with_solayer(&self, amount: f64) -> Result<String, Box<dyn std::error::Error>> {
        stake_with_solayer(self, amount).await
    }

    pub async fn get_token_data_by_address(&self, mint: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        get_token_data_by_address(mint).await
    }

    pub async fn create_webhook(
        &self,
        account_addresses: Vec<String>,
        webhook_url: String,
    ) -> Result<HeliusWebhookResponse, Box<dyn std::error::Error>> {
        create_webhook(self, account_addresses, webhook_url).await
    }
}
