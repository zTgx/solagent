use crate::actions::deploy_collection;
use crate::agent::SolAgent;
use crate::primitives::token::CollectionOptions;
use rig::{completion::ToolDefinition, tool::Tool};
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

pub struct DeployCollection<'a> {
    agent: &'a SolAgent,
    options: CollectionOptions,
}

impl<'a> DeployCollection<'a> {
    pub fn new(agent: &'a SolAgent, options: CollectionOptions) -> Self {
        DeployCollection { agent, options }
    }
}

impl<'a> Tool for DeployCollection<'a> {
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
