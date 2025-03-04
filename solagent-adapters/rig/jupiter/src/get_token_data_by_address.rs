use serde::{Deserialize, Serialize};
use solagent_core::rig::{completion::ToolDefinition, tool::Tool};
use solagent_parameters::parameters;
use solagent_plugin_jupiter::get_token_data_by_address;

#[derive(Debug, Deserialize)]
pub struct GetTokenDataArgs {
    mint: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetTokenDataOutput {
    pub data: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
#[error("GetTokenData error")]
pub struct GetTokenDataError;

#[derive(Default)]
pub struct GetTokenData;
impl GetTokenData {
    pub fn new() -> Self {
        GetTokenData {}
    }
}

impl Tool for GetTokenData {
    const NAME: &'static str = "get_token_data_by_address";

    type Error = GetTokenDataError;
    type Args = GetTokenDataArgs;
    type Output = GetTokenDataOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_token_data_by_address".to_string(),
            description: r#"
            Get the token data for a given token mint address.

            examples: [
                [
                    {
                        input: {
                            mint: "So11111111111111111111111111111111111111112",
                        },
                        explanation: "Get the current price of SOL token in USDC",
                    },
                ],
            ]
              "#
            .to_string(),
            parameters: parameters!(
                mint: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = get_token_data_by_address(&args.mint)
            .await
            .expect("get_token_data_by_address");

        Ok(GetTokenDataOutput { data })
    }
}
