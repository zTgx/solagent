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

use crate::{actions::get_token_data_by_address, parameters_json_schema};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct GetTokenDataArgs {
    mint: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetTokenDataOutput {
    pub data: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
#[error("GetTokenData error")]
pub struct GetTokenDataError;

#[derive(Default)]
pub struct GetTokenData;
impl GetTokenData {
    pub fn new() -> Self {
        GetTokenData {}
    }
}

impl Tool for GetTokenData {
    const NAME: &'static str = "get_token_data_by_address";

    type Error = GetTokenDataError;
    type Args = GetTokenDataArgs;
    type Output = GetTokenDataOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_token_data_by_address".to_string(),
            description: r#"
            Get the token data for a given token mint address.

            examples: [
                [
                    {
                        input: {
                            mint: "So11111111111111111111111111111111111111112",
                        },
                        explanation: "Get the current price of SOL token in USDC",
                    },
                ],
            ]
              "#
            .to_string(),
            parameters: parameters_json_schema!(
                mint: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = get_token_data_by_address(&args.mint).await.expect("get_token_data_by_address");

        Ok(GetTokenDataOutput { data })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for GetTokenData {
    type InitError = InitError;
    type Context = ();
    type State = ();

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(GetTokenData {})
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Get the token data for a given token mint address.".into()]
    }

    fn context(&self) -> Self::Context {}
}
