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

/// Fetches the price of a given token quoted in USDC using Jupiter API.
///
/// # Parameters
///
/// - `mint`: The token mint address as a string.
///
/// # Returns
///
/// The token data.
pub async fn get_token_data_by_address(mint: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("https://tokens.jup.ag/token/{}", mint);
    let response = reqwest::get(&url).await?;
    if !response.status().is_success() {
        return Err(format!("Failed to get token data: {}", response.status()).into());
    }

    let data: serde_json::Value = response.json().await?;
    Ok(data)
}
