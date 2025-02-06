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
use solagent_core::{
    rig::{completion::ToolDefinition, tool::Tool},
    serde_json,
};
use solagent_parameters::parameters;
use solagent_plugin_dexscreener::{get_token_data_by_ticker, JupiterTokenData};

#[derive(Debug, Deserialize)]
pub struct GetTokenDataByTickerArgs {
    ticker: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetTokenDataByTickerOutput {
    pub data: JupiterTokenData,
}

#[derive(Debug, thiserror::Error)]
#[error("GetTokenDataByTicker error")]
pub struct GetTokenDataByTickerError;

#[derive(Default)]
pub struct GetTokenDataByTicker {}

impl GetTokenDataByTicker {
    pub fn new() -> Self {
        GetTokenDataByTicker {}
    }
}

impl Tool for GetTokenDataByTicker {
    const NAME: &'static str = "get_token_data_by_ticker";

    type Error = GetTokenDataByTickerError;
    type Args = GetTokenDataByTickerArgs;
    type Output = GetTokenDataByTickerOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_token_data_by_ticker".to_string(),
            description: "Get the token data for a given token ticker".to_string(),
            parameters: parameters!(
                ticker: String
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = get_token_data_by_ticker(&args.ticker).await.expect("get_token_data_by_ticker");

        Ok(GetTokenDataByTickerOutput { data })
    }
}
