use serde::{Deserialize, Serialize};
use solagent_core::rig::{
    completion::ToolDefinition,
    tool::Tool,
};
use solagent_parameters::parameters;
use solagent_plugin_pyth::{fetch_price_by_pyth, fetch_pyth_price_feed_id};

#[derive(Debug, Deserialize)]
pub struct FetchPricePyThArgs {
    token_symbol: String,
}

#[derive(Deserialize, Serialize)]
pub struct FetchPricePyThOutput {
    pub price: f64,
}

#[derive(Debug, thiserror::Error)]
#[error("FetchPricePyTh error")]
pub struct FetchPricePyThError;

#[derive(Default)]
pub struct FetchPricePyTh;
impl FetchPricePyTh {
    pub fn new() -> Self {
        FetchPricePyTh {}
    }
}

impl Tool for FetchPricePyTh {
    const NAME: &'static str = "fetch_price_by_pyth";

    type Error = FetchPricePyThError;
    type Args = FetchPricePyThArgs;
    type Output = FetchPricePyThOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "fetch_price_by_pyth".to_string(),
            description: r#"
            
            Fetch the current price from a Pyth oracle price feed.
            
            examples: [
                [
                    {
                        input: {
                            token_symbol: "SOL", // SOL/USD price feed
                        },
                        output: {
                            status: "success",
                            price: "23.45",
                            message: "Current price: $23.45",
                        },
                        explanation: "Get the current SOL/USD price from Pyth oracle",
                    },
                ],
            ],

            "#
            .to_string(),
            parameters: parameters!(
                token_symbol: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let price_feed_id = fetch_pyth_price_feed_id(&args.token_symbol).await.expect("fetch_pyth_price_feed_id");
        let price = fetch_price_by_pyth(&price_feed_id).await.expect("fetch_price_by_pyth");

        Ok(FetchPricePyThOutput { price })
    }
}