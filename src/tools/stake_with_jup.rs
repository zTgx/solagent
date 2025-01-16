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

use crate::{actions::stake_with_jup, parameters_json_schema, SolanaAgentKit};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct StakeWithJupArgs {
    amount: f64,
}

#[derive(Deserialize, Serialize)]
pub struct StakeWithJupOutput {
    pub signature: String,
}

#[derive(Debug, thiserror::Error)]
#[error("StakeWithJup error")]
pub struct StakeWithJupError;

pub struct StakeWithJup {
    agent: Arc<SolanaAgentKit>,
}

impl StakeWithJup {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        StakeWithJup { agent }
    }
}

impl Tool for StakeWithJup {
    const NAME: &'static str = "stake_with_jup";

    type Error = StakeWithJupError;
    type Args = StakeWithJupArgs;
    type Output = StakeWithJupOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "stake_with_jup".to_string(),
            description: r#"
            Stake SOL tokens with Jupiter's liquid staking protocol to receive jupSOL
               
            examples: [
                [
                    {
                        input: {
                            amount: 1.5,
                        },
                        output: {
                            status: "success",
                            signature: "5KtPn3...",
                            message: "Successfully staked 1.5 SOL for jupSOL",
                        },
                        explanation: "Stake 1.5 SOL to receive jupSOL tokens",
                    },
                ],
            ],
                
            "#
            .to_string(),
            parameters: parameters_json_schema!(
                signature: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let amount = args.amount;
        let signature = stake_with_jup(&self.agent, amount).await.expect("stake_with_jup");

        Ok(StakeWithJupOutput { signature })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for StakeWithJup {
    type InitError = InitError;
    type Context = ();
    type State = Arc<SolanaAgentKit>;

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(StakeWithJup { agent: state })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Stake SOL tokens with Jupiter's liquid staking protocol to receive jupSOL.".into()]
    }

    fn context(&self) -> Self::Context {}
}
