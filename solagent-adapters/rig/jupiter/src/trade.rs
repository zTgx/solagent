use serde::{Deserialize, Serialize};
use solagent_core::{
    rig::{completion::ToolDefinition, tool::Tool},
    SolanaAgentKit,
};
use solagent_parameters::parameters;
use solagent_plugin_jupiter::trade;
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

            {
                input: {
                    output_mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
                    input_amount: 1,
                },
            }
            
            "#
            .to_string(),
            parameters: parameters!(
                output_mint: String,
                input_amount: f64,
                input_mint: Option<String>,
                slippage_bps: Option<u32>,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let signature = trade(
            &self.agent,
            &args.output_mint,
            args.input_amount,
            args.input_mint,
            args.slippage_bps,
        )
        .await
        .expect("trade");

        Ok(TradeOutput { signature })
    }
}
