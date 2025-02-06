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

use serde::{Deserialize, Serialize};
use solagent_core::rig::{completion::ToolDefinition, tool::Tool};
use solagent_parameters::parameters;
use solagent_plugin_story::{get_license_token, list_license_tokens, StoryBodyParams, StoryConfig};

#[derive(Deserialize)]
pub struct GetLicenseTokenArgs {
    config: StoryConfig,
    license_token_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetLicenseTokenOutput {
    pub data: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
#[error("GetLicenseToken error")]
pub struct GetLicenseTokenError;

#[derive(Default)]
pub struct GetLicenseToken {}

impl GetLicenseToken {
    pub fn new() -> Self {
        GetLicenseToken {}
    }
}

impl Tool for GetLicenseToken {
    const NAME: &'static str = "get_license_token";

    type Error = GetLicenseTokenError;
    type Args = GetLicenseTokenArgs;
    type Output = GetLicenseTokenOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_license_token".to_string(),
            description: r#"
            
            Retrieve a license token on Story protocol

            input: {
                config: {
                    api_key: xxx-xxx,
                    chain: 1516
                },
                license_token_id: "0xiii",
            },
        
            "#
            .to_string(),
            parameters: parameters!(
                config: StoryConfig,
                license_token_id: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = get_license_token(&args.config, &args.license_token_id).await.expect("get_license_token");

        Ok(GetLicenseTokenOutput { data })
    }
}

// List License Tokens

#[derive(Deserialize)]
pub struct ListLicenseTokensArgs {
    config: StoryConfig,
    body: Option<StoryBodyParams>,
}

#[derive(Deserialize, Serialize)]
pub struct ListLicenseTokensOutput {
    pub data: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
#[error("ListLicenseTokens error")]
pub struct ListLicenseTokensError;

#[derive(Default)]
pub struct ListLicenseTokens {}

impl ListLicenseTokens {
    pub fn new() -> Self {
        ListLicenseTokens {}
    }
}

impl Tool for ListLicenseTokens {
    const NAME: &'static str = "list_license_tokens";

    type Error = ListLicenseTokensError;
    type Args = ListLicenseTokensArgs;
    type Output = ListLicenseTokensOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "list_license_tokens".to_string(),
            description: r#"
            Retrieve a paginated, filtered list of LicenseTokens on Story protocol
            "#
            .to_string(),
            parameters: parameters!(
                config: StoryConfig,
                body: Option<StoryBodyParams>,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = list_license_tokens(&args.config, args.body).await.expect("list_license_tokens");

        Ok(ListLicenseTokensOutput { data })
    }
}
