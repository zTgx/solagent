use serde::{Deserialize, Serialize};
use solagent_core::{
    rig::{
        completion::ToolDefinition,
        tool::Tool,
    },
    SolanaAgentKit,
};
use solagent_parameters::parameters;
use solagent_plugin_solayer::stake_with_solayer;
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
            parameters: parameters!(
                amount: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let signature = stake_with_solayer(&self.agent, args.amount)
            .await
            .expect("stake_with_solayer");

        Ok(StakeWithSolayerOutput { signature })
    }
}
