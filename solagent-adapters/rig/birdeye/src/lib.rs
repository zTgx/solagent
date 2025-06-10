use serde::{Deserialize, Serialize};
use solagent_core::{
    rig::{completion::ToolDefinition, tool::Tool},
    SolanaAgentKit,
};
use solagent_parameters::parameters;
use solagent_plugin_birdeye::{
    get_market_data, get_token_overview, get_wallet_portfolio, TokenMarketDataResponse, TokenOverviewResponse,
    WalletPortfolioResponse,
};
use std::sync::Arc;

///////////////////////////////////////////////////////////////////////////////////////////////////////
///
/// Market Data Tool
///
/// ///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Deserialize)]
pub struct MarketDataArgs {
    address: String,
}

#[derive(Deserialize, Serialize)]
pub struct MarketDataOutput {
    pub data: TokenMarketDataResponse,
}

#[derive(Debug, thiserror::Error)]
#[error("MarketData error")]
pub struct MarketDataError;

pub struct MarketData {
    agent: Arc<SolanaAgentKit>,
}

impl MarketData {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        MarketData { agent }
    }
}

impl Tool for MarketData {
    const NAME: &'static str = "get_market_data";

    type Error = MarketDataError;
    type Args = MarketDataArgs;
    type Output = MarketDataOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_market_data".to_string(),
            description: "Get market data of single token by birdeye api".to_string(),
            parameters: parameters!(
                address: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = get_market_data(&self.agent, &args.address).await.expect("get_market_data");

        Ok(MarketDataOutput { data })
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////
///
/// Token Overview Tool
///
/// ///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Deserialize)]
pub struct TokenOverviewArgs {
    address: String,
}

#[derive(Deserialize, Serialize)]
pub struct TokenOverviewOutput {
    pub data: TokenOverviewResponse,
}

#[derive(Debug, thiserror::Error)]
#[error("MarketData error")]
pub struct TokenOverviewError;

pub struct TokenOverview {
    agent: Arc<SolanaAgentKit>,
}

impl TokenOverview {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        TokenOverview { agent }
    }
}

impl Tool for TokenOverview {
    const NAME: &'static str = "get_token_overview";

    type Error = TokenOverviewError;
    type Args = TokenOverviewArgs;
    type Output = TokenOverviewOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_token_overview".to_string(),
            description: "Get overview of a token by birdeye api".to_string(),
            parameters: parameters!(
                address: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = get_token_overview(&self.agent, &args.address).await.expect("get_token_overview");

        Ok(TokenOverviewOutput { data })
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////
///
/// Wallet portfolio Tool
///
/// ///////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Deserialize)]
pub struct WalletPortfoioArgs {
    address: String,
}

#[derive(Deserialize, Serialize)]
pub struct WalletPortfoioOutput {
    pub data: WalletPortfolioResponse,
}

#[derive(Debug, thiserror::Error)]
#[error("WalletPortfoio error")]
pub struct WalletPortfoioError;

pub struct WalletPortfoio {
    agent: Arc<SolanaAgentKit>,
}

impl WalletPortfoio {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        WalletPortfoio { agent }
    }
}

impl Tool for WalletPortfoio {
    const NAME: &'static str = "get_wallet_portfolio";

    type Error = WalletPortfoioError;
    type Args = WalletPortfoioArgs;
    type Output = WalletPortfoioOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_wallet_portfolio".to_string(),
            description: "Get wallet portfoio by birdeye api".to_string(),
            parameters: parameters!(
                address: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        if let Ok(data) = get_wallet_portfolio(&self.agent, &args.address).await {
            return Ok(WalletPortfoioOutput { data });
        }

        Err(WalletPortfoioError)
    }
}
