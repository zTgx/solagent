use serde::{Deserialize, Serialize};
use serde_json::json;
use solagent_core::rig::{completion::ToolDefinition, tool::Tool};
use solagent_parameters::parameters;
use solagent_plugin_goplus::get_token_malicious_info;

#[derive(Debug, Deserialize)]
pub struct TokenMaliciousInfoArgs {
    chain_id: String,
    address: String,
}

#[derive(Deserialize, Serialize)]
pub struct TokenMaliciousInfoOutput {
    pub data: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
#[error("TokenMaliciousInfo error")]
pub struct TokenMaliciousInfoError;

#[derive(Default)]
pub struct TokenMaliciousInfo;
impl TokenMaliciousInfo {
    pub fn new() -> Self {
        TokenMaliciousInfo {}
    }
}

impl Tool for TokenMaliciousInfo {
    const NAME: &'static str = "get_token_malicious_info";

    type Error = TokenMaliciousInfoError;
    type Args = TokenMaliciousInfoArgs;
    type Output = TokenMaliciousInfoOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_token_malicious_info".to_string(),
            description: r#"
            Check if the address is malicious

            examples: [
                [
                    {
                        input: {
                            chain_id: "1",
                            address: "So11111111111111111111111111111111111111112",
                        },
                    },
                ],
            ]
              "#
            .to_string(),
            parameters: parameters!(
                chain_id: String,
                contract_address: String
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = get_token_malicious_info(&args.chain_id, &args.address).await.expect("get_token_malicious_info");

        Ok(TokenMaliciousInfoOutput { data })
    }
}
