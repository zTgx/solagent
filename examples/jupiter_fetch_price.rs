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

use rig::{
    completion::Prompt,
    providers::gemini::{self, completion::GEMINI_1_5_PRO},
};
use solagent::{fetch_price::FetchPrice, SolanaAgentKit};

#[tokio::main]
async fn main() {
    // TODO: bug here: https://github.com/zTgx/solagent.rs/issues/1
    let token_id = "So11111111111111111111111111111111111111112";

    // let token_id = "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN";
    let price = SolanaAgentKit::fetch_price(token_id).await.unwrap();
    println!("Price: {}", price);

    let fetch_price_tool = FetchPrice;
    let client = gemini::Client::from_env();
    let agent = client
        .agent(GEMINI_1_5_PRO)
        .preamble(
            "You are an assistant here to help the user select which tool is most appropriate to perform operations.",
        )
        .max_tokens(1024)
        .tool(fetch_price_tool)
        .build();

    // call get balance tool
    let prompt = format!("fetch price of token_id {}", token_id);
    let response = agent.prompt(&prompt).await.expect("Failed to prompt Gemini");

    println!("Gemini response: {response}");
}
