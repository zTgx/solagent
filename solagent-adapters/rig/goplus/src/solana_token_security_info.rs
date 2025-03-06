use serde::{Deserialize, Serialize};
use solagent_core::rig::{completion::ToolDefinition, tool::Tool};
use solagent_parameters::parameters;
use solagent_plugin_goplus::get_solana_token_security_info;

#[derive(Debug, Deserialize)]
pub struct SolanaTokenSecurityInfoArgs {
    contract_address: String,
}

#[derive(Deserialize, Serialize)]
pub struct SolanaTokenSecurityInfoOutput {
    pub data: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
#[error("SolanaTokenSecurityInfo error")]
pub struct SolanaTokenSecurityInfoError;

#[derive(Default)]
pub struct SolanaTokenSecurityInfo;
impl SolanaTokenSecurityInfo {
    pub fn new() -> Self {
        SolanaTokenSecurityInfo {}
    }
}

impl Tool for SolanaTokenSecurityInfo {
    const NAME: &'static str = "get_solana_token_security_info";

    type Error = SolanaTokenSecurityInfoError;
    type Args = SolanaTokenSecurityInfoArgs;
    type Output = SolanaTokenSecurityInfoOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_solana_token_security_info".to_string(),
            description: r#"
            
            Token Security API for Solana (Beta).

            examples: [
                [
                    {
                        input: {
                            contract_address: "So11111111111111111111111111111111111111112",
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
        let data =
            get_solana_token_security_info(&args.contract_address).await.expect("get_solana_token_security_info");

        Ok(SolanaTokenSecurityInfoOutput { data })
    }
}
