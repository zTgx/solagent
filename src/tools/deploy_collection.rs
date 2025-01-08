use std::sync::Arc;

use crate::actions::deploy_collection;
use crate::agent::SolAgent;
use crate::primitives::token::NftMetadata;
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};

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
    const NAME: &'static str = "deploy_token";

    type Error = DeployCollectionError;
    type Args = DeployCollectionArgs;
    type Output = DeployCollectionOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "deploy_token".to_string(),
            description: r#"Deploy a new NFT collection on Solana blockchain.:
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
        let (mint_address, tx_signature) = deploy_collection(&self.agent, &self.options)
            .await
            .expect("deploy_collection");

        Ok(DeployCollectionOutput {
            mint_address,
            tx_signature,
        })
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
        Ok(DeployCollection {
            agent: state.0,
            options: state.1,
        })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Get the balance of a Solana wallet or token account.".into()]
    }

    fn context(&self) -> Self::Context {}
}
