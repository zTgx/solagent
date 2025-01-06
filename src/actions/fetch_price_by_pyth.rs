#![allow(dead_code)]

use crate::constants::PYTH_API;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Price {
    price: String,
    conf: String,
    expo: i32,
    publish_time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ParsedData {
    id: String,
    price: Price,
    ema_price: Price,
    metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
struct BinaryData {
    encoding: String,
    data: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    slot: u64,
    proof_available_time: i64,
    prev_publish_time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    binary: BinaryData,
    parsed: Vec<ParsedData>,
}

/// Fetch the price of a given price feed from Pyth.
///
/// # Parameters
/// - `price_feed_id`: Price feed ID.
///
/// # Returns
/// Latest price value from feed.
///
/// You can find priceFeedIDs here: https://www.pyth.network/developers/price-feed-ids#stable
/// get Hermes service URL from https://docs.pyth.network/price-feeds/api-instances-and-providers/hermes
pub async fn fetch_price_by_pyth(price_feed_id: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let ids = vec![price_feed_id];
    let response = reqwest::Client::new()
        .get(PYTH_API)
        .query(&ids.iter().map(|id| ("ids[]", *id)).collect::<Vec<_>>())
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(format!("Failed to fetch price: {}", response.status()).into());
    }

    let data: Response = response.json().await?;
    let parsed_data = data.parsed;
    if !parsed_data.is_empty() {
        let price_data = &parsed_data[0];
        let price_info = &price_data.price;
        let price = price_info.price.parse::<f64>().unwrap();
        let expo = price_info.expo;

        let price = price * (10.0_f64.powi(expo));
        return Ok(price);
    }

    Err("Price data not available for the given token.".into())
}
