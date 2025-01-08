use crate::{actions::get_balance, agent::SolAgent};
use crate::{parameters_json_schema, SOL_AGENT};
use rig::tool::ToolEmbedding;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct GetBalanceArgs {
    token_address: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct GetBalanceOutput {
    pub balance: f64,
}

#[derive(Debug, thiserror::Error)]
#[error("GetBalance error")]
pub struct GetBalanceError;

pub struct GetBalance<'a> {
    agent: &'a SolAgent,
}

impl<'a> GetBalance<'a> {
    pub fn new(agent: &'a SolAgent) -> Self {
        GetBalance { agent }
    }
}

impl<'a> Tool for GetBalance<'a> {
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
            parameters: parameters_json_schema!(
                token_address: String,
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

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl<'a> ToolEmbedding for GetBalance<'a> {
    type InitError = InitError;
    type Context = ();
    type State = ();

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(GetBalance { agent: &SOL_AGENT })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["call get_balance function to request".into()]
    }

    fn context(&self) -> Self::Context {}
}
