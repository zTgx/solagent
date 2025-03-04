use serde::{Deserialize, Serialize};
use solagent_core::{
    rig::{completion::ToolDefinition, tool::Tool},
    SolanaAgentKit,
};
use solagent_parameters::parameters;
use solagent_plugin_jupiter::stake_with_jup;
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
            parameters: parameters!(
                amount: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let signature = stake_with_jup(&self.agent, args.amount)
            .await
            .expect("stake_with_jup");

        Ok(StakeWithJupOutput { signature })
    }
}
