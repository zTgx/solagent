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

use solagent::{Config, SolanaAgentKit};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let config = Config { openai_api_key: Some("your_api_key".to_string()), ..Default::default() };
    let agent = Arc::new(SolanaAgentKit::new("private_key", "RPC_URL", config));
    // Task details
    let title = "Implement New Feature";
    let content = "We need to implement a new authentication system using JWT tokens";
    let requirements =
        "- Experience with Rust and JWT\n- Understanding of authentication flows\n- Test coverage required";
    let tags = vec!["rust".to_string(), "authentication".to_string(), "jwt".to_string()];
    let token_mint_address = "So11111111111111111111111111111111111111112";
    let token_amount = 1_000_000_000; // 1 SOL = 1 billion lamports

    let payer = None;

    let response = agent
        .create_gibwork_task(title, content, requirements, tags, token_mint_address, token_amount, payer)
        .await
        .unwrap();

    println!("Task created successfully!");
    println!("Task ID: {}", response.task_id);
    println!("Transaction signature: {}", response.signature);
}
