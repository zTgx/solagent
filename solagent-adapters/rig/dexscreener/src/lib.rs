use serde::{Deserialize, Serialize};
use solagent_core::{
    rig::{completion::ToolDefinition, tool::Tool},
};
use solagent_parameters::parameters;
use solagent_plugin_dexscreener::{get_token_data_by_ticker, DexTokenData};

#[derive(Debug, Deserialize)]
pub struct GetTokenDataByTickerArgs {
    ticker: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetTokenDataByTickerOutput {
    pub data: DexTokenData,
}

#[derive(Debug, thiserror::Error)]
#[error("GetTokenDataByTicker error")]
pub struct GetTokenDataByTickerError;

#[derive(Default)]
pub struct GetTokenDataByTicker {}

impl GetTokenDataByTicker {
    pub fn new() -> Self {
        GetTokenDataByTicker {}
    }
}

impl Tool for GetTokenDataByTicker {
    const NAME: &'static str = "get_token_data_by_ticker";

    type Error = GetTokenDataByTickerError;
    type Args = GetTokenDataByTickerArgs;
    type Output = GetTokenDataByTickerOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_token_data_by_ticker".to_string(),
            description: "Get the token data for a given token ticker on Dexscreener".to_string(),
            parameters: parameters!(
                ticker: String
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = get_token_data_by_ticker(&args.ticker).await.expect("get_token_data_by_ticker");

        Ok(GetTokenDataByTickerOutput { data })
    }
}
