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
    rig::{completion::ToolDefinition, tool::Tool},
    serde_json, SolanaAgentKit,
};
use solagent_parameters::parameters;
use solagent_plugin_cookie::search_tweets;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct SearchTweetsArgs {
    tweets: String,
    from: String,
    to: String,
}

#[derive(Deserialize, Serialize)]
pub struct SearchTweetsOutput {
    pub data: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
#[error("SearchTweets error")]
pub struct SearchTweetsError;

pub struct SearchTweets {
    agent: Arc<SolanaAgentKit>,
}

impl SearchTweets {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        SearchTweets { agent }
    }
}

impl Tool for SearchTweets {
    const NAME: &'static str = "search_tweets";

    type Error = SearchTweetsError;
    type Args = SearchTweetsArgs;
    type Output = SearchTweetsOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "search_tweets".to_string(),
            description: r#"
            Retrieve popular content matching search query, created in time range {from} - {to} (YYYY-MM-DD dates).
              "#
            .to_string(),
            parameters: parameters!(
                tweets: String,
                from: String,
                to: String,            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let data = search_tweets(&self.agent, &args.tweets, &args.from, &args.to).await.expect("search_tweets");

        Ok(SearchTweetsOutput { data })
    }
}
