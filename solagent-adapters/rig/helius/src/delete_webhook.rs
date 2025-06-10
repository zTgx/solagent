use serde::{Deserialize, Serialize};
use solagent_core::{
    rig::{
        completion::ToolDefinition,
        tool::Tool,
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
