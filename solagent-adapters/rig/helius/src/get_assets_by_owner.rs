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
    rig::{
        completion::ToolDefinition,
        tool::{Tool, ToolEmbedding},
    },
    SolanaAgentKit,
};
use solagent_parameters::parameters;
use solagent_plugin_helius::get_assets_by_owner;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct GetAssetsByOwnerArgs {
    owner_public_key: String,
    limit: u32,
}

#[derive(Deserialize, Serialize)]
pub struct GetAssetsByOwnerOutput {
    pub data: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
#[error("GetAssetsByOwner error")]
pub struct GetAssetsByOwnerError;

pub struct GetAssetsByOwner {
    agent: Arc<SolanaAgentKit>,
}

impl GetAssetsByOwner {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        GetAssetsByOwner { agent }
    }
}

impl Tool for GetAssetsByOwner {
    const NAME: &'static str = "get_assets_by_owner";

    type Error = GetAssetsByOwnerError;
    type Args = GetAssetsByOwnerArgs;
    type Output = GetAssetsByOwnerOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_assets_by_owner".to_string(),
            description: r#"
            
            Fetch assets owned by a specific Solana wallet address using the Helius Digital Asset Standard API

            input: {
                owner_pubkey: "4Pf8q3mHGLdkoc1M8xWZwW5q32gYmdhwC2gJ8K9EAGDX",
                limit: 10,
            },
           
            "#
            .to_string(),
            parameters: parameters!(
                owner_pubkey: String,
                limit: u32,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data =
            get_assets_by_owner(&self.agent, &args.owner_public_key, args.limit).await.expect("get_assets_by_owner");

        Ok(GetAssetsByOwnerOutput { data })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for GetAssetsByOwner {
    type InitError = InitError;
    type Context = ();
    type State = Arc<SolanaAgentKit>;

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(GetAssetsByOwner { agent: _state })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Fetch assets owned by a specific Solana wallet address using the Helius Digital Asset Standard API".into()]
    }

    fn context(&self) -> Self::Context {}
}
