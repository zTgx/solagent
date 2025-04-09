use serde::{Deserialize, Serialize};
use solagent_core::rig::{completion::ToolDefinition, tool::Tool};
use solagent_parameters::parameters;
use solagent_plugin_jupiter::fetch_price;

#[derive(Debug, Deserialize)]
pub struct FetchPriceArgs {
    token_address: String,
}

#[derive(Deserialize, Serialize)]
pub struct FetchPriceOutput {
    pub price: String,
}

#[derive(Debug, thiserror::Error)]
#[error("FetchPrice error")]
pub struct FetchPriceError;

#[derive(Default)]
pub struct FetchPrice;
impl FetchPrice {
    pub fn new() -> Self {
        FetchPrice {}
    }
}

impl Tool for FetchPrice {
    const NAME: &'static str = "fetch_price";

    type Error = FetchPriceError;
    type Args = FetchPriceArgs;
    type Output = FetchPriceOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "fetch_price".to_string(),
            description: r#"
            Fetch the current price of a Solana token in USDC using Jupiter API.

            input: {
                token_address: "So11111111111111111111111111111111111111112",
            },

            "#
            .to_string(),
            parameters: parameters!(
                token_address: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let price = fetch_price(&args.token_address).await.expect("fetch_price");

        Ok(FetchPriceOutput { price })
    }
}
