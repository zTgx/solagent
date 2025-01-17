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

use crate::{actions::stake_with_solayer, parameters_json_schema, SolanaAgentKit};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct StakeWithSolayerArgs {
    amount: f64,
}

#[derive(Deserialize, Serialize)]
pub struct StakeWithSolayerOutput {
    pub signature: String,
}

#[derive(Debug, thiserror::Error)]
#[error("StakeWithSolayer error")]
pub struct StakeWithSolayerError;

pub struct StakeWithSolayer {
    agent: Arc<SolanaAgentKit>,
}

impl StakeWithSolayer {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        StakeWithSolayer { agent }
    }
}

impl Tool for StakeWithSolayer {
    const NAME: &'static str = "stake_with_solayer";

    type Error = StakeWithSolayerError;
    type Args = StakeWithSolayerArgs;
    type Output = StakeWithSolayerOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "stake_with_solayer".to_string(),
            description: r#"
            
            Stake native SOL with Solayer's restaking protocol to receive Solayer SOL (sSOL)

            examples: [
            [
            {
                input: {
                    amount: 1.0,
                },
                output: {
                    status: "success",
                    signature: "3FgHn9...",
                    message: "Successfully staked 1.0 SOL for Solayer SOL (sSOL)",
                },
                explanation: "Stake 1.0 SOL to receive Solayer SOL (sSOL)",
            },
            ],
        ]
        
            "#
            .to_string(),
            parameters: parameters_json_schema!(
                token_address: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let signature = stake_with_solayer(&self.agent, args.amount).await.expect("stake_with_solayer");

        Ok(StakeWithSolayerOutput { signature })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for StakeWithSolayer {
    type InitError = InitError;
    type Context = ();
    type State = Arc<SolanaAgentKit>;

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(StakeWithSolayer { agent: state })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Stake native SOL with Solayer's restaking protocol to receive Solayer SOL (sSOL)".into()]
    }

    fn context(&self) -> Self::Context {}
}
