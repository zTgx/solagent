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

use crate::{actions::deploy_collection, parameters_json_schema, primitives::token::NFTMetadata, SolanaAgentKit};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct DeployCollectionArgs {
    metadata: NFTMetadata,
}

#[derive(Deserialize, Serialize)]
pub struct DeployCollectionOutput {
    pub mint_address: String,
    pub tx_signature: String,
}

#[derive(Debug, thiserror::Error)]
#[error("DeployCollection error")]
pub struct DeployCollectionError;

pub struct DeployCollection {
    agent: Arc<SolanaAgentKit>,
}

impl DeployCollection {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        DeployCollection { agent }
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
            examples: [
                [
                {
                    input: {
                        metadata: {
                            name: "My NFT",
                            uri: "https://example.com/nft.json",
                            basis_points: 500,
                        }
                    },
                    output: {
                        status: "success",
                        message: "Collection deployed successfully",
                        collectionAddress: "7nE9GvcwsqzYxmJLSrYmSB1V1YoJWVK1KWzAcWAzjXkN",
                        name: "My Collection",
                    },
                    explanation: "Deploy an NFT collection with 5% royalty",
                },
                ],
                [
                {
                    input: {
                        metadata: {
                            name: "My NFT",
                            uri: "https://example.com/nft.json",
                        }                        
                    },
                    output: {
                        status: "success",
                        message: "Collection deployed successfully",
                        collectionAddress: "8nE9GvcwsqzYxmJLSrYmSB1V1YoJWVK1KWzAcWAzjXkM",
                        name: "Basic Collection",
                    },
                    explanation: "Deploy a basic NFT collection without royalties",
                },
                ],
            ],

            "#
            .to_string(),
            parameters: parameters_json_schema!(
                metadata: NFTMetadata
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let res = deploy_collection(&self.agent, &args.metadata).await.expect("deploy_collection");

        Ok(DeployCollectionOutput { mint_address: res.mint, tx_signature: res.signature })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for DeployCollection {
    type InitError = InitError;
    type Context = ();
    type State = Arc<SolanaAgentKit>;

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(DeployCollection { agent: state })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Deploy a new NFT collection on Solana blockchain.".into()]
    }

    fn context(&self) -> Self::Context {}
}
