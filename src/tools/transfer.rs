// Copyright 2025 zTgx
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::actions::transfer;
use crate::agent::SolAgent;
use crate::parameters_json_schema;
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

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

pub struct Transfer {
    agent: Arc<SolAgent>,
}

impl Transfer {
    pub fn new(agent: Arc<SolAgent>) -> Self {
        Transfer { agent }
    }
}

impl Tool for Transfer {
    const NAME: &'static str = "transfer";

    type Error = TransferError;
    type Args = TransferArgs;
    type Output = TransferOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "transfer".to_string(),
            description:
            r#"
            Transfer tokens or SOL to another address (also called as wallet address).

            Inputs: {
                to: string, eg "8x2dR8Mpzuz2YqyZyZjUbYWKSWesBo5jMx2Q9Y86udVk" (required)
                amount: number, eg 1 (required)
                mint: Option<String>, eg "So11111111111111111111111111111111111111112" or "SENDdRQtYMWaQrBroBrJ2Q53fgVuq95CV9UPGEvpCxa" (optional)
            }    
            "#
            .to_string(),
            parameters: parameters_json_schema!(
                to: String,
                amount: f64,
                mint: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let tx = transfer(&self.agent, &args.to, args.amount, args.mint).await.expect("transfer");

        Ok(TransferOutput { tx })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for Transfer {
    type InitError = InitError;
    type Context = ();
    type State = Arc<SolAgent>;

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(Transfer { agent: _state })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Transfer tokens or SOL to another address (also called as wallet address).".into()]
    }

    fn context(&self) -> Self::Context {}
}
