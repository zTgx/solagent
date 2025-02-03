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

use serde::{Deserialize, Serialize};
use std::error::Error;

// Define the Txns struct to represent transaction information
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Txns {
    #[serde(rename = "m5")]
    m5: BuysSells,
    #[serde(rename = "h1")]
    h1: BuysSells,
    #[serde(rename = "h6")]
    h6: BuysSells,
    #[serde(rename = "h24")]
    h24: BuysSells,
}

// Define the BuysSells struct to represent the number of buys and sells
#[derive(Debug, Serialize, Deserialize, Clone)]
struct BuysSells {
    #[serde(rename = "buys")]
    buys: u64,
    #[serde(rename = "sells")]
    sells: u64,
}

// Define the Volume struct to represent trading volume
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Volume {
    #[serde(rename = "h24")]
    h24: f64,
    #[serde(rename = "h6")]
    h6: f64,
    #[serde(rename = "h1")]
    h1: f64,
    #[serde(rename = "m5")]
    m5: f64,
}

// Define the PriceChange struct to represent price changes
#[derive(Debug, Serialize, Deserialize, Clone)]
struct PriceChange {
    #[serde(rename = "h6")]
    h6: f64,
}

// Define the Liquidity struct to represent liquidity
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Liquidity {
    #[serde(rename = "usd")]
    usd: f64,
    #[serde(rename = "base")]
    base: u64,
    #[serde(rename = "quote")]
    quote: f64,
}

// Define the Token struct to represent token information
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Token {
    #[serde(rename = "address")]
    address: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "symbol")]
    symbol: String,
}

// Define the Pair struct to represent trading pair information
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Pair {
    #[serde(rename = "chainId")]
    chain_id: String,
    #[serde(rename = "dexId")]
    dex_id: String,
    #[serde(rename = "url")]
    url: String,
    #[serde(rename = "pairAddress")]
    pair_address: String,
    #[serde(rename = "labels")]
    labels: Option<Vec<String>>,
    #[serde(rename = "baseToken")]
    base_token: Token,
    #[serde(rename = "quoteToken")]
    quote_token: Token,
    #[serde(rename = "priceNative")]
    price_native: String,
    #[serde(rename = "priceUsd")]
    price_usd: String,
    #[serde(rename = "txns")]
    txns: Txns,
    #[serde(rename = "volume")]
    volume: Volume,
    #[serde(rename = "priceChange")]
    price_change: PriceChange,
    #[serde(rename = "liquidity")]
    liquidity: Liquidity,
    #[serde(rename = "fdv")]
    fdv: u64,
    #[serde(rename = "marketCap")]
    market_cap: u64,
    #[serde(rename = "pairCreatedAt")]
    pair_created_at: Option<u64>,
}

// Define the Root struct to represent the root structure of the JSON
#[derive(Debug, Serialize, Deserialize, Clone)]
struct SearchTokenData {
    #[serde(rename = "schemaVersion")]
    schema_version: String,
    #[serde(rename = "pairs")]
    pairs: Vec<Pair>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JupiterTokenData {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub tags: Vec<String>,
    pub logo_uri: String,
    pub daily_volume: f64,
    pub freeze_authority: Option<String>,
    pub mint_authority: Option<String>,
    pub permanent_delegate: Option<String>,
    pub extensions: Extensions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Extensions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coingecko_id: Option<String>,
}

/// Get the token data for a given token ticker.
///
/// # Parameters
///
/// - `ticker`: Ticker of the token, e.g. 'USDC'
///
/// # Returns
///
/// A `Result`
pub async fn get_token_data_by_ticker(ticker: &str) -> Result<JupiterTokenData, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let address = get_token_address_from_ticker(&client, ticker).await?;

    get_token_data_by_address(&client, &address).await
}

async fn get_token_address_from_ticker(client: &reqwest::Client, ticker: &str) -> Result<String, Box<dyn Error>> {
    let url = format!("https://api.dexscreener.com/latest/dex/search?q=${}", ticker);
    let response = client.get(&url).send().await?;
    if !response.status().is_success() {
        return Err(format!("Failed to get token address from ticker: {}", response.status()).into());
    }

    let data: SearchTokenData = response.json().await?;

    let mut solana_pairs: Vec<Pair> = data.pairs.iter().filter(|&pair| pair.chain_id == "solana").cloned().collect();

    solana_pairs.sort_by_key(|pair| std::cmp::Reverse(pair.fdv));

    let filtered_pairs: Vec<Pair> = solana_pairs
        .into_iter()
        .filter(|pair| pair.base_token.symbol.to_lowercase() == ticker.to_lowercase())
        .collect();

    filtered_pairs.into_iter().next().map(|pair| pair.base_token.address).ok_or("get address error".into())
}

async fn get_token_data_by_address(
    client: &reqwest::Client,
    address: &str,
) -> Result<JupiterTokenData, Box<dyn Error>> {
    let url = format!("https://tokens.jup.ag/token/${}", address);
    let response = client.get(&url).send().await?;
    if !response.status().is_success() {
        return Err(format!("Failed to get token address from ticker: {}", response.status()).into());
    }

    let data: JupiterTokenData = response.json().await?;
    Ok(data)
}
