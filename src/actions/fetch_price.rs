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

#![allow(dead_code)]

use crate::primitives::constants::JUP_PRICE_V2;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct PriceResponse {
    data: std::collections::HashMap<String, TokenData>,
}

#[derive(Deserialize, Debug)]
struct TokenData {
    id: Option<String>,

    #[serde(rename = "type")]
    typed: Option<String>,
    price: Option<String>,
}

/// Fetches the price of a given token quoted in USDC using Jupiter API.
///
/// # Parameters
///
/// - `token_id`: The token mint address as a string.
///
/// # Returns
///
/// The price of the token quoted in USDC as a string.
pub async fn fetch_price(token_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("{}{}", JUP_PRICE_V2, token_id);
    let response = reqwest::get(&url).await?;
    if !response.status().is_success() {
        return Err(format!("Failed to fetch price: {}", response.status()).into());
    }

    let data: PriceResponse = response.json().await?;
    // Get the price for the given token_id
    if let Some(token_data) = data.data.get(token_id) {
        if let Some(price) = &token_data.price {
            return Ok(price.clone());
        }
    }

    Err("Price data not available for the given token.".into())
}
