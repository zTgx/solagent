use crate::{actions::fetch_price, parameters_json_schema};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct FetchPriceArgs {
    token_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct FetchPriceOutput {
    pub price: String,
}

#[derive(Debug, thiserror::Error)]
#[error("FetchPrice error")]
pub struct FetchPriceError;

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
            description: r#"Fetch the current price of a Solana token in USDC using Jupiter API
                input: {
                    token_id: "",
                },
            "#
            .to_string(),
            parameters: parameters_json_schema!(
                token_id: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let token_id = args.token_id;
        let price = fetch_price(&token_id).await.expect("fetch_price");

        Ok(FetchPriceOutput { price })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for FetchPrice {
    type InitError = InitError;
    type Context = ();
    type State = ();

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(FetchPrice {})
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Fetch the current price of a Solana token in USDC using Jupiter API".into()]
    }

    fn context(&self) -> Self::Context {}
}
