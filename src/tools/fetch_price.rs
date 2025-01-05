use crate::actions::fetch_price;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct FetchPriceArgs {}

#[derive(Deserialize, Serialize)]
pub struct FetchPriceOutput {
    pub price: String,
}

#[derive(Debug, thiserror::Error)]
#[error("FetchPrice error")]
pub struct FetchPriceError;

pub struct FetchPrice<'a> {
    token_id: &'a str,
}

impl<'a> FetchPrice<'a> {
    pub fn new(token_id: &'a str) -> Self {
        FetchPrice { token_id }
    }
}

impl<'a> Tool for FetchPrice<'a> {
    const NAME: &'static str = "fetch_price";

    type Error = FetchPriceError;
    type Args = FetchPriceArgs;
    type Output = FetchPriceOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "fetch_price".to_string(),
            description: "Fetch the current price of a Solana token in USDC using Jupiter API"
                .to_string(),
            parameters: serde_json::Value::Null,
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let price = fetch_price(self.token_id).await.expect("fetch_price");

        Ok(FetchPriceOutput { price })
    }
}
