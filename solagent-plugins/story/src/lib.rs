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

mod transaction;
pub use transaction::*;

mod license_tokens;
pub use license_tokens::*;

const STORY_API_URL: &str = "https://staging-api.storyprotocol.net/api/v2";

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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
