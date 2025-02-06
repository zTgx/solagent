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
use solagent_core::rig::{completion::ToolDefinition, tool::Tool};
use solagent_parameters::parameters;
use solagent_plugin_goplus::get_token_security_info;

#[derive(Debug, Deserialize)]
pub struct TokenSecurityInfoArgs {
    chain_id: String,
    contract_address: String,
}

#[derive(Deserialize, Serialize)]
pub struct TokenSecurityInfoOutput {
    pub data: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
#[error("TokenSecurityInfo error")]
pub struct TokenSecurityInfoError;

#[derive(Default)]
pub struct TokenSecurityInfo;
impl TokenSecurityInfo {
    pub fn new() -> Self {
        TokenSecurityInfo {}
    }
}

impl Tool for TokenSecurityInfo {
    const NAME: &'static str = "get_token_security_info";

    type Error = TokenSecurityInfoError;
    type Args = TokenSecurityInfoArgs;
    type Output = TokenSecurityInfoOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_token_security_info".to_string(),
            description: r#"
            Get token security information

            examples: [
                [
                    {
                        input: {
                            chain_id: "1",
                            contract_address: "So11111111111111111111111111111111111111112",
                        },
                    },
                ],
            ]
              "#
            .to_string(),
            parameters: parameters!(
                chain_id: String,
                contract_address: String
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data =
            get_token_security_info(&args.chain_id, &args.contract_address).await.expect("get_token_security_info");

        Ok(TokenSecurityInfoOutput { data })
    }
}
