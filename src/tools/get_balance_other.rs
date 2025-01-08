use std::sync::Arc;

use crate::parameters_json_schema;
use crate::{actions::get_balance_other, agent::SolAgent};
use rig::tool::ToolEmbedding;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_sdk::pubkey::Pubkey;

#[derive(Deserialize)]
pub struct GetBalanceOtherArgs {
    wallet_address: Pubkey,
    token_address: Option<Pubkey>,
}

#[derive(Deserialize, Serialize)]
pub struct GetBalanceOtherOutput {
    pub balance: f64,
}

#[derive(Debug, thiserror::Error)]
#[error("GetBalanceOther error")]
pub struct GetBalanceOtherError;

pub struct GetBalanceOther {
    agent: Arc<SolAgent>,
}

impl GetBalanceOther {
    pub fn new(agent: Arc<SolAgent>) -> Self {
        GetBalanceOther { agent }
    }
}

impl Tool for GetBalanceOther {
    const NAME: &'static str = "get_balance_other";

    type Error = GetBalanceOtherError;
    type Args = GetBalanceOtherArgs;
    type Output = GetBalanceOtherOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_balance_other".to_string(),
            description: r#"Get the balance of a Solana wallet or token account which is different from the agent's wallet.
  If no tokenAddress is provided, the SOL balance of the wallet will be returned.
  Inputs ( input is a JSON string ):
  walletAddress: string, eg "GDEkQF7UMr7RLv1KQKMtm8E2w3iafxJLtyXu3HVQZnME" (required)
  tokenAddress: string, eg "SENDdRQtYMWaQrBroBrJ2Q53fgVuq95CV9UPGEvpCxa" (optional)"#.to_string(),
            parameters: parameters_json_schema!(
                wallet_address: object,
                token_address: object,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let wallet_address = args.wallet_address;
        let token_address = args.token_address;
        let balance = get_balance_other(&self.agent, wallet_address, token_address)
            .await
            .expect("get_balance_other");

        Ok(GetBalanceOtherOutput { balance })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for GetBalanceOther {
    type InitError = InitError;
    type Context = ();
    type State = Arc<SolAgent>;

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(GetBalanceOther { agent: _state })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Get the balance of a Solana wallet or token account.".into()]
    }

    fn context(&self) -> Self::Context {}
}
