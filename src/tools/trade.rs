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

use crate::{actions::trade, parameters_json_schema, SolanaAgentKit};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct TradeArgs {
    output_mint: String,
    input_amount: f64,
    input_mint: Option<String>,
    slippage_bps: Option<u32>,
}

#[derive(Deserialize, Serialize)]
pub struct TradeOutput {
    pub signature: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Trade error")]
pub struct TradeError;

pub struct Trade {
    agent: Arc<SolanaAgentKit>,
}

impl Trade {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        Trade { agent }
    }
}

impl Tool for Trade {
    const NAME: &'static str = "trade";

    type Error = TradeError;
    type Args = TradeArgs;
    type Output = TradeOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "trade".to_string(),
            description: r#"
            This tool can be used to swap tokens to another token (It uses Jupiter Exchange).

            [
                {
                    input: {
                        outputMint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
                        inputAmount: 1,
                    },
                    explanation: "Swap 1 SOL for USDC",
                },
                ],
                [
                {
                    input: {
                        outputMint: "So11111111111111111111111111111111111111112",
                        inputAmount: 100,
                        inputMint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
                        slippageBps: 100,
                    },
                    explanation: "Swap 100 USDC for SOL with 1% slippage",
                },
            ],
            
            "#
            .to_string(),
            parameters: parameters_json_schema!(
                output_mint: String,
                input_amount: f64,
                input_mint: String,
                slippage_bps: u32,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let signature = trade(&self.agent, &args.output_mint, args.input_amount, args.input_mint, args.slippage_bps)
            .await
            .expect("trade");

        Ok(TradeOutput { signature })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for Trade {
    type InitError = InitError;
    type Context = ();
    type State = Arc<SolanaAgentKit>;

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(Trade { agent: _state })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["This tool can be used to swap tokens to another token (It uses Jupiter Exchange).".into()]
    }

    fn context(&self) -> Self::Context {}
}
