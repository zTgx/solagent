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
use solagent_core::{
    rig::{
        completion::ToolDefinition,
        tool::{Tool, ToolEmbedding},
    },
    SolanaAgentKit,
};
use solagent_parameters::parameters;
use solagent_plugin_helius::delete_webhook;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct DeleteWebHookArgs {
    webhook_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteWebHookOutput {
    pub data: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
#[error("DeleteWebHook error")]
pub struct DeleteWebHookError;

pub struct DeleteWebHook {
    agent: Arc<SolanaAgentKit>,
}

impl DeleteWebHook {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        DeleteWebHook { agent }
    }
}

impl Tool for DeleteWebHook {
    const NAME: &'static str = "delete_webhook";

    type Error = DeleteWebHookError;
    type Args = DeleteWebHookArgs;
    type Output = DeleteWebHookOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "delete_webhook".to_string(),
            description: r#"
            
            Deletes a Helius webhook by its unique ID

            input: {
              webhook_id: "webhook_123",
            },
           
            "#
            .to_string(),
            parameters: parameters!(
                webhook_id: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = delete_webhook(&self.agent, &args.webhook_id).await.expect("delete_webhook");

        Ok(DeleteWebHookOutput { data })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for DeleteWebHook {
    type InitError = InitError;
    type Context = ();
    type State = Arc<SolanaAgentKit>;

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(DeleteWebHook { agent: _state })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Deletes a Helius webhook by its unique ID".into()]
    }

    fn context(&self) -> Self::Context {}
}
