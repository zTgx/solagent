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

use solagent_core::{solana_sdk::signature::Keypair, Config, SolanaAgentKit};
use solagent_plugin_gibwork::create_gibwork_task;

#[tokio::main]
async fn main() {
    // Create a new keypair
    let keypair = Keypair::new();
    // Encode the secret key to base58
    let private_key = keypair.to_base58_string();

    let config = Config { cookie_api_key: Some("".to_string()), ..Default::default() };
    let agent = SolanaAgentKit::new(&private_key, "https://api.devnet.solana.com", config);

    // Task details
    let title = "Implement New Feature";
    let content = "We need to implement a new authentication system using JWT tokens";
    let requirements =
        "- Experience with Rust and JWT\n- Understanding of authentication flows\n- Test coverage required";
    let tags = vec!["rust".to_string(), "authentication".to_string(), "jwt".to_string()];
    let token_mint_address = "So11111111111111111111111111111111111111112";
    let token_amount = 1_000_000_000; // 1 SOL = 1 billion lamports

    let payer = None;

    let response =
        create_gibwork_task(&agent, title, content, requirements, tags, token_mint_address, token_amount, payer)
            .await
            .unwrap();

    println!("Task created successfully!");
    println!("Task ID: {}", response.task_id);
    println!("Transaction signature: {}", response.signature);
}
