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

use serde_json::json;

use crate::{StoryBodyParams, StoryConfig, STORY_API_URL};

/// Retrieve a LicenseToken
///
/// # Arguments
///
/// * `config` - API Config
/// * `license_token_id` - License Token ID
///
/// # Returns
///
/// License Token data
pub async fn get_license_token(
    config: &StoryConfig,
    license_token_id: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("{}/licenses/tokens/{}", STORY_API_URL, license_token_id);

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("X-Api-Key", &config.api_key)
        .header("X-Chain", config.chain)
        .header("accept", "application/json")
        .send()
        .await?;

    let data: serde_json::Value = response.json().await?;
    Ok(data)
}

/// Retrieve a paginated, filtered list of LicenseTokens
///
/// # Arguments
///
/// * `config` - API Config
/// * `story_body_params` - Query Parameters must be wrapped in options object and may be empty. OrderBy must be blockNumber, resourceType or empty.
///
/// # Returns
///
/// list of LicenseTokens
pub async fn list_license_tokens(
    config: &StoryConfig,
    body: Option<StoryBodyParams>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("{}/licenses/tokens", STORY_API_URL);

    let client = reqwest::Client::new();

    let mut request_builder = client
        .post(url)
        .header("X-Api-Key", &config.api_key)
        .header("X-Chain", config.chain)
        .header("accept", "application/json")
        .header("content-type", "application/json");

    if let Some(body_params) = body {
        let request_data = json!({
            "options": body_params
        });
        let json_body = serde_json::to_string(&request_data)?;

        request_builder = request_builder.body(json_body);
    }

    let response = request_builder.send().await?;
    let data: serde_json::Value = response.json().await?;
    Ok(data)
}
