use serde::{Deserialize, Serialize};
use solagent_core::{
    rig::{
        completion::ToolDefinition,
        tool::Tool,
    },
    SolanaAgentKit,
};
use solagent_parameters::parameters;
use solagent_plugin_helius::transaction_parse;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct TransactionParseArgs {
    transaction_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct TransactionParseOutput {
    pub data: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
#[error("TransactionParse error")]
pub struct TransactionParseError;

pub struct TransactionParse {
    agent: Arc<SolanaAgentKit>,
}

impl TransactionParse {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        TransactionParse { agent }
    }
}

impl Tool for TransactionParse {
    const NAME: &'static str = "transaction_parse";

    type Error = TransactionParseError;
    type Args = TransactionParseArgs;
    type Output = TransactionParseOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "transaction_parse".to_string(),
            description: r#"
            
            Parse a Solana transaction to retrieve detailed information using the Helius Enhanced Transactions API

            input: {
                transaction_id: "tx123",
            },
           
            "#
            .to_string(),
            parameters: parameters!(
                transaction_id: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = transaction_parse(&self.agent, &args.transaction_id).await.expect("transaction_parse");

        Ok(TransactionParseOutput { data })
    }
}