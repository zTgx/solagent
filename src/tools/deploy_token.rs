use crate::actions::deploy_token;
use crate::agent::SolAgent;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};

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
    pub tx: String,
}

#[derive(Debug, thiserror::Error)]
#[error("DeployToken error")]
pub struct DeployTokenError;

pub struct DeployToken<'a> {
    agent: &'a SolAgent,
}

impl<'a> DeployToken<'a> {
    pub fn new(agent: &'a SolAgent) -> Self {
        DeployToken { agent }
    }
}

impl<'a> Tool for DeployToken<'a> {
    const NAME: &'static str = "deploy_token";

    type Error = DeployTokenError;
    type Args = DeployTokenArgs;
    type Output = DeployTokenOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "deploy_token".to_string(),
            description:
                r#"Deploy a new SPL token on the Solana blockchain with specified parameters:
            input: {
                name: "My Token",
                uri: "https://example.com/token.json",
                symbol: "MTK",
                decimals: 9,
                initialSupply: 1000000,
            },
            "#
                .to_string(),
            parameters: serde_json::Value::Null,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let tx = deploy_token(
            &self.agent,
            args.name,
            args.uri,
            args.symbol,
            args.decimals,
            args.initial_supply,
        )
        .await
        .expect("deploy_token");

        Ok(DeployTokenOutput { tx: tx.to_string() })
    }
}
