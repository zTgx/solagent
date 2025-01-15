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

use crate::primitives::constants::RUGCHECK_URL;
use serde_json::Value;

/// Fetches a detailed report for a specific token.
///
/// # Parameters
///
/// - `mint` - The mint address of the token.
///
/// # Returns
/// Token detailed report.
///
/// # Errors
/// Throws an error if the API call fails.
pub async fn fetch_detailed_report(mint: String) -> Result<Value, Box<dyn std::error::Error>> {
    let url = format!("{}/tokens/{}/report", RUGCHECK_URL, mint);

    let response = reqwest::get(&url).await?;
    if !response.status().is_success() {
        return Err(format!("HTTP error! status: {}", response.status()).into());
    }

    let data: serde_json::Value = response.json().await?;
    Ok(data)
}
