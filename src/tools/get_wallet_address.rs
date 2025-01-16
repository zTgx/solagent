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

use crate::{actions::get_wallet_address, SolanaAgentKit};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct GetWalletAddressArgs {}

#[derive(Deserialize, Serialize)]
pub struct GetWalletAddressOutput {
    pub address: String,
}

#[derive(Debug, thiserror::Error)]
#[error("GetWalletAddress error")]
pub struct GetWalletAddressError;

pub struct GetWalletAddress {
    agent: Arc<SolanaAgentKit>,
}

impl GetWalletAddress {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        GetWalletAddress { agent }
    }
}

impl Tool for GetWalletAddress {
    const NAME: &'static str = "get_wallet_address";

    type Error = GetWalletAddressError;
    type Args = GetWalletAddressArgs;
    type Output = GetWalletAddressOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_wallet_address".to_string(),
            description: r#"
            Get wallet address of the agent".to_string(),
            
            examples: [
                [
                    {
                        input: {},
                        output: {
                            status: "success",
                            address: "0x1234567890abcdef",
                        },
                        explanation: "The agent's wallet address is 0x1234567890abcdef",
                    },
                ],
            ]
            "#
            .to_string(),
            parameters: serde_json::Value::Null,
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let address = get_wallet_address(&self.agent);

        Ok(GetWalletAddressOutput { address })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for GetWalletAddress {
    type InitError = InitError;
    type Context = ();
    type State = Arc<SolanaAgentKit>;

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(GetWalletAddress { agent: _state })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Get wallet address of the agent".into()]
    }

    fn context(&self) -> Self::Context {}
}
