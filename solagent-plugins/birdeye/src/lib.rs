use anyhow::Result;
use solagent_core::SolanaAgentKit;

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
pub async fn get_token_overview(agent: &SolanaAgentKit, address: &str) -> Result<TokenOverviewResponse> {
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
/// A `Result` TokenMarketDataResponse
pub async fn get_market_data(agent: &SolanaAgentKit, address: &str) -> Result<TokenMarketDataResponse> {
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
        .json::<TokenMarketDataResponse>()
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
pub async fn get_wallet_portfolio(agent: &SolanaAgentKit, wallet_address: &str) -> Result<WalletPortfolioResponse> {
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
pub async fn get_token_holders(
    agent: &SolanaAgentKit,
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

/// Get metadata of single token
///
/// # Parameters
///
/// * `agent` - An instance of SolanaAgentKit (with .config.birdeye_api_key)
///
/// - `address`: Address of a token
///
/// # Returns
///
/// A `Result` TokenMetadataResponse
pub async fn get_token_metadata(agent: &SolanaAgentKit, address: &str) -> Result<TokenMetadataResponse> {
    let api_key = agent
        .config
        .birdeye_api_key
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Missing Birdeye API key in agent.config.birdeye_api_key"))?;

    let client = reqwest::Client::new();
    let url = format!("{}/defi/v3/token/meta-data/single", BIRDEYE_URL);

    let resp = client
        .get(url)
        .query(&[("address", address)])
        .header("X-API-KEY", api_key)
        .header("accept", "application/json")
        .header("x-chain", "solana")
        .send()
        .await?
        .json::<TokenMetadataResponse>()
        .await?;

    Ok(resp)
}

/// Get price update of a token.
///
/// # Parameters
///
/// * `agent` - An instance of SolanaAgentKit (with .config.birdeye_api_key)
///
/// - `address`: Address of a token
///
/// # Returns
///
/// A `Result` TokenPriceResponse
pub async fn get_token_price(agent: &SolanaAgentKit, address: &str) -> Result<TokenPriceResponse> {
    let api_key = agent
        .config
        .birdeye_api_key
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Missing Birdeye API key in agent.config.birdeye_api_key"))?;

    let client = reqwest::Client::new();
    let url = format!("{}/defi/price", BIRDEYE_URL);

    let resp = client
        .get(url)
        .query(&[("address", address)])
        .header("X-API-KEY", api_key)
        .header("accept", "application/json")
        .header("x-chain", "solana")
        .send()
        .await?
        .json::<TokenPriceResponse>()
        .await?;

    Ok(resp)
}

/// Get price and volume updates of a token
///
/// # Parameters
///
/// * `agent` - An instance of SolanaAgentKit (with .config.birdeye_api_key)
///
/// - `query_params`: TokenPriceVolumeQueryParams
///
/// # Returns
///
/// A `Result` TokenPriceVolumeResponse
pub async fn get_token_price_volume(
    agent: &SolanaAgentKit,
    query_params: TokenPriceVolumeQueryParams,
) -> Result<TokenPriceVolumeResponse> {
    let api_key = agent
        .config
        .birdeye_api_key
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Missing Birdeye API key in agent.config.birdeye_api_key"))?;

    let client = reqwest::Client::new();
    let url = format!("{}/defi/price_volume/single", BIRDEYE_URL);

    let address = query_params.address;
    let vh = query_params.vh;

    let resp = client
        .get(url)
        .query(&[("address", address), ("type", vh)])
        .header("X-API-KEY", api_key)
        .header("accept", "application/json")
        .header("x-chain", "solana")
        .send()
        .await?
        .json::<TokenPriceVolumeResponse>()
        .await?;

    Ok(resp)
}

/// Retrieve a dynamic and up-to-date list of trending tokens based on specified sorting criteria.
///
/// # Parameters
///
/// * `agent` - An instance of SolanaAgentKit (with .config.birdeye_api_key)
///
/// - `query_params`: TokenTrendingQueryParams
///
/// # Returns
///
/// A `Result` TokenTrendingResponse
pub async fn get_token_trending(
    agent: &SolanaAgentKit,
    query_params: TokenTrendingQueryParams,
) -> Result<TokenTrendingResponse> {
    let api_key = agent
        .config
        .birdeye_api_key
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Missing Birdeye API key in agent.config.birdeye_api_key"))?;

    let client = reqwest::Client::new();
    let url = format!("{}/defi/token_trending", BIRDEYE_URL);

    let sort_by = query_params.sort_by;
    let sort_type = query_params.sort_type;
    let offset = query_params.offset.unwrap_or(0);
    let limit = query_params.limit.unwrap_or(20);

    let resp = client
        .get(url)
        .query(&[
            ("sort_by", sort_by),
            ("sort_type", sort_type),
            ("offset", offset.to_string()),
            ("limit", limit.to_string()),
        ])
        .header("X-API-KEY", api_key)
        .header("accept", "application/json")
        .header("x-chain", "solana")
        .send()
        .await?
        .json::<TokenTrendingResponse>()
        .await?;

    Ok(resp)
}
