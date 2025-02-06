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

use crate::STORY_API_URL;
use serde::{Deserialize, Serialize};

pub struct StoryConfig {
    pub api_key: String,
    pub chain: u32,
}

impl StoryConfig {
    pub fn new(api_key: &str, chain: u32) -> Self {
        StoryConfig { api_key: api_key.to_string(), chain }
    }
}

#[derive(Serialize, Deserialize)]
pub struct StoryPaginationObject {
    pub after: String,
    pub before: String,
    pub limit: u32,
}

#[derive(Serialize, Deserialize)]
pub struct StoryBodyWhereObject {
    pub block_number: String,
    pub id: String,
    pub ip_id: String,
    pub resource_type: String,
    pub transaction_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct StoryBodyParams {
    order_by: String,
    order_direction: String,
    pagination: StoryPaginationObject,
    where_obj: StoryBodyWhereObject,
}

/// Retrieve a Transaction
///
/// # Arguments
///
/// * `config` - API Config
/// * `trx_id` - Transaction ID
///
/// # Returns
///
/// Transaction signature
pub async fn get_a_transaction(
    config: &StoryConfig,
    trx_id: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("{}/{}", STORY_API_URL, trx_id);

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

/// Retrieve a paginated, filtered list of Transactions
///
/// # Arguments
///
/// * `config` - API Config
/// * `story_body_params` - Query Parameters must be wrapped in options object and may be empty. OrderBy must be blockNumber, resourceType or empty.
///
/// # Returns
///
/// Transaction signature
pub async fn list_transactions(
    config: &StoryConfig,
    body: Option<StoryBodyParams>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    query_transactions(STORY_API_URL, config, body).await
}

/// Retrieve a paginated, filtered list of Latest Transactions
///
/// # Arguments
///
/// * `config` - API Config
/// * `story_body_params` - Query Parameters must be wrapped in options object and may be empty. OrderBy must be blockNumber, resourceType or empty.
///
/// # Returns
///
/// Transaction signature
pub async fn list_latest_transactions(
    config: &StoryConfig,
    body: Option<StoryBodyParams>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("{}/latest", STORY_API_URL);
    query_transactions(&url, config, body).await
}

async fn query_transactions(
    url: &str,
    config: &StoryConfig,
    body: Option<StoryBodyParams>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut request_builder = client
        .post(url)
        .header("X-Api-Key", &config.api_key)
        .header("X-Chain", config.chain)
        .header("accept", "application/json")
        .header("content-type", "application/json");

    if let Some(body_params) = body {
        let json_body = serde_json::to_string(&body_params)?;
        request_builder = request_builder.body(json_body);
    }

    let response = request_builder.send().await?;
    let data: serde_json::Value = response.json().await?;
    Ok(data)
}
