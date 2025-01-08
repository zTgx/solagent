use crate::agent::SolAgent;
use crate::{actions::get_tps, SOL_AGENT};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GetTpsArgs {}

#[derive(Deserialize, Serialize)]
pub struct GetTpsOutput {
    pub tps: f64,
}

#[derive(Debug, thiserror::Error)]
#[error("GetTps error")]
pub struct GetTpsError;

pub struct GetTps<'a> {
    agent: &'a SolAgent,
}

impl<'a> GetTps<'a> {
    pub fn new() -> Self {
        GetTps { agent: &SOL_AGENT }
    }
}

impl<'a> Tool for GetTps<'a> {
    const NAME: &'static str = "get_tps";

    type Error = GetTpsError;
    type Args = GetTpsArgs;
    type Output = GetTpsOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_tps".to_string(),
            description: "Get the current transactions per second (TPS) of the Solana network"
                .to_string(),
            parameters: serde_json::Value::Null,
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let tps = get_tps(&self.agent).await.expect("tps");

        Ok(GetTpsOutput { tps })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl<'a> ToolEmbedding for GetTps<'a> {
    type InitError = InitError;
    type Context = ();
    type State = ();

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(GetTps { agent: &SOL_AGENT })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Get wallet address of the agent".into()]
    }

    fn context(&self) -> Self::Context {}
}
