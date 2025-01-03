use crate::{actions::request_faucet_funds, agent::SolAgent};
use rig::{completion::ToolDefinition, tool::Tool};
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

pub struct RequestFaucetFunds<'a> {
    agent: &'a SolAgent,
}
impl<'a> RequestFaucetFunds<'a> {
    pub fn new(agent: &'a SolAgent) -> Self {
        RequestFaucetFunds { agent }
    }
}

impl<'a> Tool for RequestFaucetFunds<'a> {
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
