use serde::{Deserialize, Serialize};
use solagent_core::{
    rig::{
        completion::ToolDefinition,
        tool::Tool,
    },
    SolanaAgentKit,
};
use solagent_parameters::parameters;
use solagent_plugin_helius::{create_webhook, HeliusWebhookResponse};
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
                account_addresses: [
                    "BVdNLvyG2DNiWAXBE9qAmc4MTQXymd5Bzfo9xrQSUzVP",
                    "Eo2ciguhMLmcTWXELuEQPdu7DWZt67LHXb2rdHZUbot7",
                ],
                webhook_url: "https://yourdomain.com/webhook",
            },
           
            "#
            .to_string(),
            parameters: parameters!(
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