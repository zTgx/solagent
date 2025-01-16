// Copyright 2025 zTgx
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
    actions::launch_token_pumpfun,
    parameters_json_schema,
    primitives::pumpfun::{PumpFunTokenOptions, PumpfunTokenResponse},
    SolanaAgentKit,
};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct LaunchPumpfunTokenArgs {
    token_name: String,
    token_symbol: String,
    description: String,
    image_url: String,
    options: Option<PumpFunTokenOptions>,
}

#[derive(Deserialize, Serialize)]
pub struct LaunchPumpfunTokenOutput {
    pub res: PumpfunTokenResponse,
}

#[derive(Debug, thiserror::Error)]
#[error("LaunchPumpfunToken error")]
pub struct LaunchPumpfunTokenError;

pub struct LaunchPumpfunToken {
    agent: Arc<SolanaAgentKit>,
}

impl LaunchPumpfunToken {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        LaunchPumpfunToken { agent }
    }
}

impl Tool for LaunchPumpfunToken {
    const NAME: &'static str = "launch_token_pumpfun";

    type Error = LaunchPumpfunTokenError;
    type Args = LaunchPumpfunTokenArgs;
    type Output = LaunchPumpfunTokenOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "launch_token_pumpfun".to_string(),
            description: r#"
            Launch a new token on Pump.fun with customizable metadata and initial liquidity.
            do not use this tool for any other purpose, or for creating SPL tokens.
            If the user asks you to chose the parameters, you should generate valid values.
            For generating the image, you can use the solana_create_image tool.

            examples: [
                [
                    {
                        input: {
                            token_name: "Sample Token",
                            token_symbol: "SMPL",
                            description: "A sample token for demonstration",
                            image_url: "https://example.com/token.png",
                            options: {
                                twitter: "@sampletoken",
                                telegram: "t.me/sampletoken",
                                website: "https://sampletoken.com",
                                initial_liquidity_sol: 0.1,
                                slippage_bps: 10,
                                priority_fee: 0.0001,                            
                            }
                        },
                        output: {
                            status: "success",
                            signature: "2ZE7Rz...",
                            mint: "7nxQB...",
                            metadataUri: "https://arweave.net/...",
                            message: "Successfully launched token on Pump.fun",
                        },

                        explanation: "Launch a new token with custom metadata and 0.1 SOL initial liquidity",
                    },
                ],
            ]
            "#
            .to_string(),
            parameters: parameters_json_schema!(
                token_address: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let res = launch_token_pumpfun(
            &self.agent,
            &args.token_name,
            &args.token_symbol,
            &args.description,
            &args.image_url,
            args.options,
        )
        .await
        .expect("launch_token_pumpfun");

        Ok(LaunchPumpfunTokenOutput { res })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for LaunchPumpfunToken {
    type InitError = InitError;
    type Context = ();
    type State = Arc<SolanaAgentKit>;

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(LaunchPumpfunToken { agent: state })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Launch a new token on Pump.fun with customizable metadata and initial liquidity.".into()]
    }

    fn context(&self) -> Self::Context {}
}
