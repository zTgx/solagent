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

use crate::{actions::get_token_malicious_info, parameters_json_schema};
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct TokenMaliciousInfoArgs {
    chain_id: String,
    address: String,
}

#[derive(Deserialize, Serialize)]
pub struct TokenMaliciousInfoOutput {
    pub data: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
#[error("TokenMaliciousInfo error")]
pub struct TokenMaliciousInfoError;

#[derive(Default)]
pub struct TokenMaliciousInfo;
impl TokenMaliciousInfo {
    pub fn new() -> Self {
        TokenMaliciousInfo {}
    }
}

impl Tool for TokenMaliciousInfo {
    const NAME: &'static str = "get_token_malicious_info";

    type Error = TokenMaliciousInfoError;
    type Args = TokenMaliciousInfoArgs;
    type Output = TokenMaliciousInfoOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_token_malicious_info".to_string(),
            description: r#"
            Check if the address is malicious

            examples: [
                [
                    {
                        input: {
                            chain_id: "1",
                            address: "So11111111111111111111111111111111111111112",
                        },
                    },
                ],
            ]
              "#
            .to_string(),
            parameters: parameters_json_schema!(
                chain_id: String,
                contract_address: String
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = get_token_malicious_info(&args.chain_id, &args.address).await.expect("get_token_malicious_info");

        Ok(TokenMaliciousInfoOutput { data })
    }
}
