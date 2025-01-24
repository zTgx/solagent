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
    actions::{create_webhook, HeliusWebhookResponse},
    parameters_json_schema, SolanaAgentKit,
};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CreateWebHookArgs {
    account_addresses: Vec<String>,
    webhook_url: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateWebHookOutput {
    pub data: HeliusWebhookResponse,
}

#[derive(Debug, thiserror::Error)]
#[error("CreateWebHook error")]
pub struct CreateWebHookError;

pub struct CreateWebHook {
    agent: Arc<SolanaAgentKit>,
}

impl CreateWebHook {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        CreateWebHook { agent }
    }
}

impl Tool for CreateWebHook {
    const NAME: &'static str = "create_webhook";

    type Error = CreateWebHookError;
    type Args = CreateWebHookArgs;
    type Output = CreateWebHookOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "create_webhook".to_string(),
            description: r#"
            
            Creates a new webhook in the Helius system to monitor transactions for specified account addresses

            input: {
                accountAddresses: [
                    "BVdNLvyG2DNiWAXBE9qAmc4MTQXymd5Bzfo9xrQSUzVP",
                    "Eo2ciguhMLmcTWXELuEQPdu7DWZt67LHXb2rdHZUbot7",
                ],
                webhookURL: "https://yourdomain.com/webhook",
            },
           
            "#
            .to_string(),
            parameters: parameters_json_schema!(
                account_addresses: Vec<String>,
                webhook_url: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let account_addresses = args.account_addresses;
        let webhook_url = args.webhook_url;
        let data = create_webhook(&self.agent, account_addresses, webhook_url).await.expect("create_webhook");

        Ok(CreateWebHookOutput { data })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for CreateWebHook {
    type InitError = InitError;
    type Context = ();
    type State = Arc<SolanaAgentKit>;

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(CreateWebHook { agent: _state })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Creates a new webhook in the Helius system to monitor transactions for specified account addresses".into()]
    }

    fn context(&self) -> Self::Context {}
}
