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

use crate::{actions::transfer, parameters_json_schema, SolanaAgentKit};
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
    agent: Arc<SolanaAgentKit>,
}

impl Transfer {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
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
            description: r#"
            Transfer tokens or SOL to another address (also called as wallet address).

            examples: [
                [
                    {
                        input: {
                            to: "8x2dR8Mpzuz2YqyZyZjUbYWKSWesBo5jMx2Q9Y86udVk",
                            amount: 1,
                        },
                        output: {
                            status: "success",
                            message: "Transfer completed successfully",
                            amount: 1,
                            recipient: "8x2dR8Mpzuz2YqyZyZjUbYWKSWesBo5jMx2Q9Y86udVk",
                            token: "SOL",
                            transaction:
                                "5UfgJ5vVZxUxefDGqzqkVLHzHxVTyYH9StYyHKgvHYmXJgqJKxEqy9k4Rz9LpXrHF9kUZB7",
                        },
                        explanation: "Transfer 1 SOL to the recipient address",
                    },
                ],
                [
                    {
                        input: {
                            to: "8x2dR8Mpzuz2YqyZyZjUbYWKSWesBo5jMx2Q9Y86udVk",
                            amount: 100,
                            mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
                        },
                        output: {
                            status: "success",
                            message: "Transfer completed successfully",
                            amount: 100,
                            recipient: "8x2dR8Mpzuz2YqyZyZjUbYWKSWesBo5jMx2Q9Y86udVk",
                            token: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
                            transaction:
                                "4VfgJ5vVZxUxefDGqzqkVLHzHxVTyYH9StYyHKgvHYmXJgqJKxEqy9k4Rz9LpXrHF9kUZB7",
                        },
                        explanation: "Transfer 100 USDC tokens to the recipient address",
                    },
                ],
            ],
 
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
    type State = Arc<SolanaAgentKit>;

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(Transfer { agent: _state })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Transfer tokens or SOL to another address (also called as wallet address).".into()]
    }

    fn context(&self) -> Self::Context {}
}
