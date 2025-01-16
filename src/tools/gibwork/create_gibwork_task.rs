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

use crate::{
    actions::{create_gibwork_task, GibworkCreateTaskResponse},
    parameters_json_schema, SolanaAgentKit,
};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CreateGibworkTaskArgs {
    title: String,
    content: String,
    requirements: String,
    tags: Vec<String>,
    token_mint_address: String,
    token_amount: u64,
    payer: Option<Pubkey>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateGibworkTaskOutput {
    pub data: GibworkCreateTaskResponse,
}

#[derive(Debug, thiserror::Error)]
#[error("CreateGibworkTask error")]
pub struct CreateGibworkTaskError;

pub struct CreateGibworkTask {
    agent: Arc<SolanaAgentKit>,
}

impl CreateGibworkTask {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        CreateGibworkTask { agent }
    }
}

impl Tool for CreateGibworkTask {
    const NAME: &'static str = "create_gibwork_task";

    type Error = CreateGibworkTaskError;
    type Args = CreateGibworkTaskArgs;
    type Output = CreateGibworkTaskOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "create_gibwork_task".to_string(),
            description: r#"
            Create a new task on the Gibwork platform with payment in SPL tokens.
            
            examples: [
                [
                    {
                        input: {
                            title: "Build a Solana dApp",
                            content: "Create a simple Solana dApp with React frontend",
                            requirements: "Experience with Rust and React",
                            tags: ["solana", "rust", "react"],
                            tokenMintAddress: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
                            tokenAmount: 100,
                        },
                        output: {
                            status: "success",
                            taskId: "task_123",
                            signature: "3YKpM1...",
                            message: "Successfully created task: Build a Solana dApp",
                        },
                        explanation: "Create a new task on Gibwork with 100 USDC payment",
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
        let data = create_gibwork_task(
            &self.agent,
            &args.title,
            &args.content,
            &args.requirements,
            args.tags,
            &args.token_mint_address,
            args.token_amount,
            args.payer,
        )
        .await
        .expect("create_gibwork_task");

        Ok(CreateGibworkTaskOutput { data })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for CreateGibworkTask {
    type InitError = InitError;
    type Context = ();
    type State = Arc<SolanaAgentKit>;

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(CreateGibworkTask { agent: state })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Create a new task on the Gibwork platform with payment in SPL tokens".into()]
    }

    fn context(&self) -> Self::Context {}
}
