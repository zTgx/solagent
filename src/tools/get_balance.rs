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

use crate::{actions::get_balance, parameters_json_schema, SolanaAgentKit};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct GetBalanceArgs {
    token_address: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct GetBalanceOutput {
    pub balance: f64,
}

#[derive(Debug, thiserror::Error)]
#[error("GetBalance error")]
pub struct GetBalanceError;

pub struct GetBalance {
    agent: Arc<SolanaAgentKit>,
}

impl GetBalance {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        GetBalance { agent }
    }
}

impl Tool for GetBalance {
    const NAME: &'static str = "get_balance";

    type Error = GetBalanceError;
    type Args = GetBalanceArgs;
    type Output = GetBalanceOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_balance".to_string(),
            description: r#"
            Get the balance of a Solana wallet or token account.
            If you want to get the balance of your wallet, you don't need to provide the tokenAddress.
            If no tokenAddress is provided, the balance will be in SOL.
            
            examples: [
                [
                    {
                        input: {},
                        output: {
                            status: "success",
                            balance: "100",
                            token: "SOL",
                        },
                        explanation: "Get SOL balance of the wallet",
                    },
                ],
                [
                    {
                        input: {
                            tokenAddress: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
                        },
                        output: {
                            status: "success",
                            balance: "1000",
                            token: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
                        },
                        explanation: "Get USDC token balance",
                    },
                ],
            ]
            "#
            .to_string(),
            parameters: parameters_json_schema!(
                token_address: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let token_address = args.token_address;
        let balance = get_balance(&self.agent, token_address).await.expect("get_balance");

        Ok(GetBalanceOutput { balance })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for GetBalance {
    type InitError = InitError;
    type Context = ();
    type State = Arc<SolanaAgentKit>;

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(GetBalance { agent: state })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Get the balance of a Solana wallet or token account.".into()]
    }

    fn context(&self) -> Self::Context {}
}
