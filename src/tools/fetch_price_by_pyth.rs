use crate::{
    actions::{fetch_price_by_pyth, fetch_pyth_price_feed_id},
    parameters_json_schema,
};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

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
            description: r#"Fetch the current price from a Pyth oracle price feed
                input: {
                    token_symbol: "SOL", // SOL/USD price feed
                },
            "#
            .to_string(),
            parameters: parameters_json_schema!(
                token_symbol: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let token_symbol = args.token_symbol;
        let price_feed_id = fetch_pyth_price_feed_id(&token_symbol)
            .await
            .expect("fetch_pyth_price_feed_id");
        let price = fetch_price_by_pyth(&price_feed_id)
            .await
            .expect("fetch_price_by_pyth");

        Ok(FetchPricePyThOutput { price })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for FetchPricePyTh {
    type InitError = InitError;
    type Context = ();
    type State = ();

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(FetchPricePyTh {})
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Fetch the current price from a Pyth oracle price feed".into()]
    }

    fn context(&self) -> Self::Context {}
}
