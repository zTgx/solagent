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

use crate::{actions::transaction_parse, parameters_json_schema, SolanaAgentKit};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
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
            parameters: parameters_json_schema!(
                transaction_id: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = transaction_parse(&self.agent, &args.transaction_id).await.expect("transaction_parse");

        Ok(TransactionParseOutput { data })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for TransactionParse {
    type InitError = InitError;
    type Context = ();
    type State = Arc<SolanaAgentKit>;

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(TransactionParse { agent: _state })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Parse a Solana transaction to retrieve detailed information using the Helius Enhanced Transactions API"
            .into()]
    }

    fn context(&self) -> Self::Context {}
}
