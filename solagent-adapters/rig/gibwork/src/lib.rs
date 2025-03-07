use serde::{Deserialize, Serialize};
use solagent_core::{
    rig::{completion::ToolDefinition, tool::Tool},
    solana_sdk::pubkey::Pubkey,
    SolanaAgentKit,
};
use solagent_parameters::parameters;
use solagent_plugin_gibwork::{create_gibwork_task, GibworkCreateTaskResponse};
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
            parameters: parameters!(
                title: String,
                content: String,
                requirements: String,
                tags: Vec<String>,
                token_mint_address: String,
                token_amount: u64,
                payer: Option<Pubkey>,
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
