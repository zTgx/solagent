use serde::{Deserialize, Serialize};
use solagent_core::{
    rig::{
        completion::ToolDefinition,
        tool::Tool,
    },
    SolanaAgentKit,
};
use solagent_parameters::parameters;
use solagent_plugin_helius::{get_webhook, HeliusWebhookIdResponse};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct GetWebHookArgs {
    webhook_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetWebHookOutput {
    pub data: HeliusWebhookIdResponse,
}

#[derive(Debug, thiserror::Error)]
#[error("GetWebHook error")]
pub struct GetWebHookError;

pub struct GetWebHook {
    agent: Arc<SolanaAgentKit>,
}

impl GetWebHook {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        GetWebHook { agent }
    }
}

impl Tool for GetWebHook {
    const NAME: &'static str = "get_webhook";

    type Error = GetWebHookError;
    type Args = GetWebHookArgs;
    type Output = GetWebHookOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_webhook".to_string(),
            description: r#"
            
            Retrieves details of a Helius webhook by its unique ID

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
        let data = get_webhook(&self.agent, &args.webhook_id).await.expect("get_webhook");

        Ok(GetWebHookOutput { data })
    }
}