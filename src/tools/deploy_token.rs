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

use crate::actions::deploy_token;
use crate::SolanaAgentKit;
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct DeployTokenArgs {
    pub name: String,
    pub uri: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_supply: Option<u64>,
}

#[derive(Deserialize, Serialize)]
pub struct DeployTokenOutput {
    pub mint_address: String,
    pub tx_signature: String,
}

#[derive(Debug, thiserror::Error)]
#[error("DeployToken error")]
pub struct DeployTokenError;

pub struct DeployToken {
    agent: Arc<SolanaAgentKit>,
}

impl DeployToken {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        DeployToken { agent }
    }
}

impl Tool for DeployToken {
    const NAME: &'static str = "deploy_token";

    type Error = DeployTokenError;
    type Args = DeployTokenArgs;
    type Output = DeployTokenOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "deploy_token".to_string(),
            description: r#"
            Deploy a new SPL token on the Solana blockchain with specified parameters:

            examples: [
                [
                {
                    input: {
                        name: "My Token",
                        uri: "https://example.com/token.json",
                        symbol: "MTK",
                        decimals: 9,
                        initialSupply: 1000000,
                    },
                    output: {
                        mint: "7nE9GvcwsqzYxmJLSrYmSB1V1YoJWVK1KWzAcWAzjXkN",
                        status: "success",
                        message: "Token deployed successfully",
                    },
                    explanation: "Deploy a token with initial supply and metadata",
                },
                ],
                [
                {
                    input: {
                        name: "Basic Token",
                        uri: "https://example.com/basic.json",
                        symbol: "BASIC",
                    },
                    output: {
                        mint: "8nE9GvcwsqzYxmJLSrYmSB1V1YoJWVK1KWzAcWAzjXkM",
                        status: "success",
                        message: "Token deployed successfully",
                    },
                    explanation: "Deploy a basic token with minimal parameters",
                },
                ],
            ],
            
            "#
            .to_string(),
            parameters: serde_json::Value::Null,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let res = deploy_token(&self.agent, args.name, args.uri, args.symbol, args.decimals, args.initial_supply)
            .await
            .expect("deploy_token");

        Ok(DeployTokenOutput { mint_address: res.mint, tx_signature: res.signature })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for DeployToken {
    type InitError = InitError;
    type Context = ();
    type State = Arc<SolanaAgentKit>;

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(DeployToken { agent: _state })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Deploy a new SPL token on the Solana blockchain with specified parameters.".into()]
    }

    fn context(&self) -> Self::Context {}
}
