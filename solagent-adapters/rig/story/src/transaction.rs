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
};
use solagent_plugin_story::{
    get_a_transaction, list_latest_transactions, list_transactions, StoryBodyParams, StoryConfig,
};

#[derive(Deserialize)]
pub struct GetTransactionArgs {
    config: StoryConfig,
    trx_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetTransactionOutput {
    pub data: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
#[error("GetTransaction error")]
pub struct GetTransactionError;

#[derive(Default)]
pub struct GetTransaction {}

impl GetTransaction {
    pub fn new() -> Self {
        GetTransaction {}
    }
}

impl Tool for GetTransaction {
    const NAME: &'static str = "get_a_transaction";

    type Error = GetTransactionError;
    type Args = GetTransactionArgs;
    type Output = GetTransactionOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_a_transaction".to_string(),
            description: r#"
            
            Retrieve a Transaction on Story protocol

            input: {
                config: {
                    api_key: xxx-xxx,
                    chain: 1516
                },
                trx_id: "0xiii",
            },
        
            "#
            .to_string(),
            parameters: parameters_json_schema!(
                config: StoryConfig,
                trx_id: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = get_a_transaction(&args.config, &args.trx_id).await.expect("get_a_transaction");

        Ok(GetTransactionOutput { data })
    }
}

// List transactions

#[derive(Deserialize)]
pub struct ListTransactionsArgs {
    config: StoryConfig,
    body: Option<StoryBodyParams>,
}

#[derive(Deserialize, Serialize)]
pub struct ListTransactionsOutput {
    pub data: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
#[error("ListTransactions error")]
pub struct ListTransactionsError;

#[derive(Default)]
pub struct ListTransactions {}

impl ListTransactions {
    pub fn new() -> Self {
        ListTransactions {}
    }
}

impl Tool for ListTransactions {
    const NAME: &'static str = "list_transactions";

    type Error = ListTransactionsError;
    type Args = ListTransactionsArgs;
    type Output = ListTransactionsOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "list_transactions".to_string(),
            description: r#"
            Retrieve a paginated, filtered list of Transactions
            "#
            .to_string(),
            parameters: parameters_json_schema!(
                config: StoryConfig,
                body: Option<StoryBodyParams>,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = list_transactions(&args.config, args.body).await.expect("list_transactions");

        Ok(ListTransactionsOutput { data })
    }
}

// list of Latest Transactions

#[derive(Deserialize)]
pub struct ListLatestTransactionsArgs {
    config: StoryConfig,
    body: Option<StoryBodyParams>,
}

#[derive(Deserialize, Serialize)]
pub struct ListLatestTransactionsOutput {
    pub data: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
#[error("ListTransactions error")]
pub struct ListLatestTransactionsError;

#[derive(Default)]
pub struct ListLatestTransactions {}

impl ListLatestTransactions {
    pub fn new() -> Self {
        ListLatestTransactions {}
    }
}

impl Tool for ListLatestTransactions {
    const NAME: &'static str = "list_latest_transactions";

    type Error = ListLatestTransactionsError;
    type Args = ListLatestTransactionsArgs;
    type Output = ListLatestTransactionsOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "list_latest_transactions".to_string(),
            description: r#"
            Retrieve a paginated, filtered list of Latest Transactions
            "#
            .to_string(),
            parameters: parameters_json_schema!(
                config: StoryConfig,
                body: Option<StoryBodyParams>,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = list_latest_transactions(&args.config, args.body).await.expect("list_latest_transactions");

        Ok(ListLatestTransactionsOutput { data })
    }
}
