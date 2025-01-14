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

/// This example requires that you have the [`ollama`](https://ollama.com) server running locally.
/// More details: https://wale-e.github.io/ai/agent/framework/2025/01/01/hello-world-rig.html
use rig::{completion::Prompt, providers};
use solagent::fetch_price::FetchPrice;

#[tokio::main]
async fn main() -> Result<(), String> {
    let token_id = "So11111111111111111111111111111111111111112";
    let prompt = format!("fetch price of token_id {}", token_id);

    // Create an OpenAI client with a custom base url, a local ollama endpoint
    // The API Key is unnecessary for most local endpoints
    let client = providers::openai::Client::from_url("ollama", "http://localhost:11434/v1");
    // Create agent with a single context prompt
    let comedian_agent = client
        .agent("llama3.2")
        .preamble(
            "You are an assistant here to help the user select which tool is most appropriate to perform operations.",
        )
        .tool(FetchPrice)
        .build();

    // Prompt the agent and print the response
    let response = comedian_agent.prompt(&prompt).await.unwrap();
    println!("{}", response);

    Ok(())
}
