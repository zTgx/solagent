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

use crate::{actions::request_faucet_funds, SolanaAgentKit};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct RequestFaucetFundsArgs;

#[derive(Deserialize, Serialize)]
pub struct RequestFaucetFundsOutput {
    pub tx: String,
}

#[derive(Debug, thiserror::Error)]
#[error("RequestFaucetFunds error")]
pub struct RequestFaucetFundsError;

pub struct RequestFaucetFunds {
    agent: Arc<SolanaAgentKit>,
}
impl RequestFaucetFunds {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        RequestFaucetFunds { agent }
    }
}

impl Tool for RequestFaucetFunds {
    const NAME: &'static str = "request_faucet_funds";

    type Error = RequestFaucetFundsError;
    type Args = RequestFaucetFundsArgs;
    type Output = RequestFaucetFundsOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "request_faucet_funds".to_string(),
            description: r#"
            Request SOL from Solana faucet (devnet/testnet only)
            
            examples: [
                [
                {
                    input: {},
                    output: {
                        status: "success",
                        message: "Successfully requested faucet funds",
                        network: "devnet.solana.com",
                    },
                    explanation: "Request SOL from the devnet faucet",
                },
                ],
            ],
            "#
            .to_string(),
            parameters: serde_json::Value::Null,
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let tx = request_faucet_funds(&self.agent).await.expect("request_faucet_funds");

        Ok(RequestFaucetFundsOutput { tx })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for RequestFaucetFunds {
    type InitError = InitError;
    type Context = ();
    type State = Arc<SolanaAgentKit>;

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(RequestFaucetFunds { agent: _state })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Request SOL from Solana faucet (devnet/testnet only)".into()]
    }

    fn context(&self) -> Self::Context {}
}
