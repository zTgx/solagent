use crate::agent::SolAgent;
use crate::parameters_json_schema;
use crate::{actions::transfer, agent::SOL_AGENT};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct TransferArgs {
    pub to: String,
    pub amount: u64,
    pub mint: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TransferOutput {
    pub tx: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Transfer error")]
pub struct TransferError;

pub struct Transfer<'a> {
    agent: &'a SolAgent,
}

impl<'a> Transfer<'a> {
    pub fn new() -> Self {
        Transfer { agent: &SOL_AGENT }
    }
}

impl<'a> Tool for Transfer<'a> {
    const NAME: &'static str = "transfer";

    type Error = TransferError;
    type Args = TransferArgs;
    type Output = TransferOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "transfer".to_string(),
            description:
                r#"
                Transfer tokens or SOL to another address ( also called as wallet address ).

                Inputs:
                to: string, eg "8x2dR8Mpzuz2YqyZyZjUbYWKSWesBo5jMx2Q9Y86udVk" (required)
                amount: number, eg 1 (required)
                mint: Option<String>, eg "So11111111111111111111111111111111111111112" or "SENDdRQtYMWaQrBroBrJ2Q53fgVuq95CV9UPGEvpCxa" (optional)"#
                .to_string(),
            parameters: parameters_json_schema!(
                to: String,
                amount: f64,
                mint: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let tx = transfer(&self.agent, &args.to, args.amount, args.mint)
            .await
            .expect("deploy_token");

        Ok(TransferOutput { tx })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl<'a> ToolEmbedding for Transfer<'a> {
    type InitError = InitError;
    type Context = ();
    type State = ();

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(Transfer { agent: &SOL_AGENT })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Transfer tokens or SOL to another address ( also called as wallet address ).".into()]
    }

    fn context(&self) -> Self::Context {}
}
