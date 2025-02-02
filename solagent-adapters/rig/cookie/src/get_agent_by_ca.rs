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
use solagent_plugin_cookie::get_agent_by_ca;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct GetAgentByCaArgs {
    contract_address: String,
    interval: Option<u32>,
}

#[derive(Deserialize, Serialize)]
pub struct GetAgentByCaOutput {
    pub data: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
#[error("GetAgentByCa error")]
pub struct GetAgentByCaError;

pub struct GetAgentByCa {
    agent: Arc<SolanaAgentKit>,
}

impl GetAgentByCa {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        GetAgentByCa { agent }
    }
}

impl Tool for GetAgentByCa {
    const NAME: &'static str = "get_agent_by_ca";

    type Error = GetAgentByCaError;
    type Args = GetAgentByCaArgs;
    type Output = GetAgentByCaOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_agent_by_ca".to_string(),
            description: r#"
            Retrieve agent details in specified interval by one of its tokens contract address.
              "#
            .to_string(),
            parameters: parameters_json_schema!(
                contract_address: String,
                interval: Option<u32>,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = get_agent_by_ca(&self.agent, &args.contract_address, args.interval).await.expect("get_agent_by_ca");

        Ok(GetAgentByCaOutput { data })
    }
}
