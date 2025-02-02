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
    parameters_json_schema,
    rig::{completion::ToolDefinition, tool::Tool},
    serde_json, SolanaAgentKit,
};
use solagent_plugin_cookie::get_agent_by_name;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct GetAgentByTwitterNameArgs {
    twitter_name: String,
    interval: Option<u32>,
}

#[derive(Deserialize, Serialize)]
pub struct GetAgentByTwitterNameOutput {
    pub data: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
#[error("GetAgentByTwitterName error")]
pub struct GetAgentByTwitterNameError;

pub struct GetAgentByTwitterName {
    agent: Arc<SolanaAgentKit>,
}

impl GetAgentByTwitterName {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        GetAgentByTwitterName { agent }
    }
}

impl Tool for GetAgentByTwitterName {
    const NAME: &'static str = "get_agent_by_name";

    type Error = GetAgentByTwitterNameError;
    type Args = GetAgentByTwitterNameArgs;
    type Output = GetAgentByTwitterNameOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_agent_by_name".to_string(),
            description: r#"
            Retrieve agent details in specified interval by twitter username.
              "#
            .to_string(),
            parameters: parameters_json_schema!(
                twitter_name: String,
                interval: Option<u32>,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = get_agent_by_name(&self.agent, &args.twitter_name, args.interval).await.expect("get_agent_by_name");

        Ok(GetAgentByTwitterNameOutput { data })
    }
}
