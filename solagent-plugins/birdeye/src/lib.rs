use anyhow::Result;
use solagent_core::{IWallet, SolanaAgentKit};

mod primitive;
pub use primitive::*;

/// Get overview of a token.
///
/// # Parameters
///
/// * `agent` - An instance of SolanaAgentKit (with .config.birdeye_api_key)
///
/// - `address`: Address of a token
///
/// # Returns
///
/// A `Result` TokenOverviewResponse
pub async fn get_token_overview<W: IWallet>(agent: &SolanaAgentKit<W>, address: &str) -> Result<TokenOverviewResponse> {
    let api_key = agent
        .config
        .birdeye_api_key
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Missing Birdeye API key in agent.config.birdeye_api_key"))?;

    let client = reqwest::Client::new();
    let url = format!("{}/defi/token_overview", BIRDEYE_URL);

    let resp = client
        .get(url)
        .query(&[("address", address)])
        .header("X-API-KEY", api_key)
        .header("accept", "application/json")
        .header("x-chain", "solana")
        .send()
        .await?
        .json::<TokenOverviewResponse>()
        .await?;

    Ok(resp)
}

/// Get market data of single token
///
/// # Parameters
///
/// * `agent` - An instance of SolanaAgentKit
///
/// - `address`: Address of a token
///
/// # Returns
///
/// A `Result`
pub async fn get_market_data<W: IWallet>(agent: &SolanaAgentKit<W>, address: &str) -> Result<MarketDataResponse> {
    let api_key = agent
        .config
        .birdeye_api_key
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Missing Birdeye API key in agent.config.birdeye_api_key"))?;

    let client = reqwest::Client::new();
    let url = format!("{}/defi/v3/token/market-data", BIRDEYE_URL);

    let resp = client
        .get(url)
        .query(&[("address", address)])
        .header("X-API-KEY", api_key)
        .header("accept", "application/json")
        .header("x-chain", "solana")
        .send()
        .await?
        .json::<MarketDataResponse>()
        .await?;

    Ok(resp)
}

/// Get Wallet Portfolio
///
/// # Parameters
///
/// * `agent` - An instance of SolanaAgentKit
///
/// - `address`: Address of a wallet
///
/// # Returns
///
/// A `WalletPortfolioResponse`
pub async fn get_wallet_portfolio<W: IWallet>(
    agent: &SolanaAgentKit<W>,
    wallet_address: &str,
) -> Result<WalletPortfolioResponse> {
    let api_key = agent
        .config
        .birdeye_api_key
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Missing Birdeye API key in agent.config.birdeye_api_key"))?;

    let client = reqwest::Client::new();
    let url = format!("{}/v1/wallet/token_list", BIRDEYE_URL);

    let response = client
        .get(&url)
        .query(&[("wallet", wallet_address)])
        .header("accept", "application/json")
        .header("X-API-KEY", api_key)
        .header("x-chain", "solana")
        .send()
        .await?
        .json::<WalletPortfolioResponse>()
        .await?;

    Ok(response)
}

/// Get top holder list of the given token
///
/// # Parameters
///
/// * `agent` - An instance of SolanaAgentKit (with .config.birdeye_api_key)
///
/// - `params`: Query Params
///
/// # Returns
///
/// A `Result` TokenHolderResponse
pub async fn get_token_holders<W: IWallet>(
    agent: &SolanaAgentKit<W>,
    query_params: TokenHolderQueryParams,
) -> Result<TokenHolderResponse> {
    let api_key = agent
        .config
        .birdeye_api_key
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Missing Birdeye API key in agent.config.birdeye_api_key"))?;

    let client = reqwest::Client::new();
    let url = format!("{}/defi/v3/token/holder", BIRDEYE_URL);

    let address = query_params.address;
    let offset = query_params.offset.unwrap_or(0);
    let limit = query_params.limit.unwrap_or(100);

    let resp = client
        .get(url)
        .query(&[("address", address), ("offset", offset.to_string()), ("limit", limit.to_string())])
        .header("X-API-KEY", api_key)
        .header("accept", "application/json")
        .header("x-chain", "solana")
        .send()
        .await?
        .json::<TokenHolderResponse>()
        .await?;

    Ok(resp)
}
