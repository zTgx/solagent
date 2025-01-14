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

use crate::actions::deploy_collection;
use crate::agent::SolAgent;
use crate::primitives::token::NftMetadata;
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct DeployCollectionArgs {}

#[derive(Deserialize, Serialize)]
pub struct DeployCollectionOutput {
    pub mint_address: String,
    pub tx_signature: String,
}

#[derive(Debug, thiserror::Error)]
#[error("DeployCollection error")]
pub struct DeployCollectionError;

pub struct DeployCollection {
    agent: Arc<SolAgent>,
    options: NftMetadata,
}

impl DeployCollection {
    pub fn new(agent: Arc<SolAgent>, options: NftMetadata) -> Self {
        DeployCollection { agent, options }
    }
}

impl Tool for DeployCollection {
    const NAME: &'static str = "deploy_collection";

    type Error = DeployCollectionError;
    type Args = DeployCollectionArgs;
    type Output = DeployCollectionOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "deploy_collection".to_string(),
            description: r#"
            Deploy a new NFT collection on Solana blockchain.:
            input: {
                name: "My Collection",
                uri: "https://example.com/collection.json",
                royaltyBasisPoints: 500,
            },
            "#
            .to_string(),
            parameters: serde_json::Value::Null,
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let res = deploy_collection(&self.agent, &self.options).await.expect("deploy_collection");

        Ok(DeployCollectionOutput { mint_address: res.mint, tx_signature: res.signature })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for DeployCollection {
    type InitError = InitError;
    type Context = ();
    type State = (Arc<SolAgent>, NftMetadata);

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(DeployCollection { agent: state.0, options: state.1 })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Deploy a new NFT collection on Solana blockchain.".into()]
    }

    fn context(&self) -> Self::Context {}
}
