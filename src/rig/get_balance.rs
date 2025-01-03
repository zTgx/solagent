use crate::{agent::SolAgent, tools::get_balance};
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_sdk::pubkey::Pubkey;

use crate::json_schema;

#[derive(Deserialize)]
pub struct GetBalanceArgs {
    token_address: Option<Pubkey>,
}

#[derive(Deserialize, Serialize)]
pub struct GetBalanceOutput {
    pub balance: f64,
}

#[derive(Debug, thiserror::Error)]
#[error("GetBalance error")]
pub struct GetBalanceError;

pub struct GetBalance {
    agent: SolAgent,
}

impl GetBalance {
    pub fn new(agent: SolAgent) -> Self {
        GetBalance { agent }
    }
}

impl Tool for GetBalance {
    const NAME: &'static str = "get_balance";

    type Error = GetBalanceError;
    type Args = GetBalanceArgs;
    type Output = GetBalanceOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_balance".to_string(),
            description: "Get the balance of a Solana wallet or token account.
  If you want to get the balance of your wallet, you don't need to provide the tokenAddress.
  If no tokenAddress is provided, the balance will be in SOL."
                .to_string(),
            parameters: json_schema!(
                token_address: string,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let token_address = args.token_address;
        let balance = get_balance(&self.agent, token_address)
            .await
            .expect("get_balance");

        Ok(GetBalanceOutput { balance })
    }
}
