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
    providers::gemini::{self, completion::GEMINI_1_5_FLASH},
};
use solagent::pyth_fetch_price::FetchPricePyTh;

#[tokio::main]
async fn main() {
    let token_symbol = "SOL";

    let fetch_price_tool = FetchPricePyTh;
    let client = gemini::Client::from_env();
    let agent = client
        .agent(GEMINI_1_5_FLASH)
        .preamble(
            "You are an assistant here to help the user select which tool is most appropriate to perform operations.",
        )
        .max_tokens(1024)
        .tool(fetch_price_tool)
        .build();

    let prompt = format!("fetch price of token symbol {}", token_symbol);
    let response = agent.prompt(&prompt).await.expect("Failed to prompt Gemini");

    println!("Gemini response: {response}");
}
