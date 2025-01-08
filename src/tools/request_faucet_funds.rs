use std::sync::Arc;

use crate::{actions::request_faucet_funds, agent::SolAgent};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::parameters_json_schema;

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
    agent: Arc<SolAgent>,
}
impl RequestFaucetFunds {
    pub fn new(agent: Arc<SolAgent>) -> Self {
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
            description: "Request SOL from Solana faucet (devnet/testnet only)".to_string(),
            parameters: parameters_json_schema!(
                token_address: string,
            ),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let tx = request_faucet_funds(&self.agent)
            .await
            .expect("request_faucet_funds");

        Ok(RequestFaucetFundsOutput { tx })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for RequestFaucetFunds {
    type InitError = InitError;
    type Context = ();
    type State = Arc<SolAgent>;

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(RequestFaucetFunds { agent: _state })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Request SOL from Solana faucet (devnet/testnet only)".into()]
    }

    fn context(&self) -> Self::Context {}
}
