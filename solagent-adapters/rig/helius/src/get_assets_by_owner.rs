use serde::{Deserialize, Serialize};
use solagent_core::{
    rig::{
        completion::ToolDefinition,
        tool::Tool,
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