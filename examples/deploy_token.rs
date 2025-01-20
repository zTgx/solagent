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

/// Example on devnet
/// Mint: 3kvSrsPwtYi6RkWymJocQcezwiDpqMfDjWazYAaibDmY
#[tokio::main]
async fn main() {
    let name = "Solagent".to_string();
    let uri = "solagent.rs".to_string();
    let symbol = "SOLA".to_string();
    let decimals = 1;
    let initial_supply = 1_000_000_000_u64;

    let config = Config { openai_api_key: Some("your_api_key".to_string()), ..Default::default() };
    let agent = Arc::new(SolanaAgentKit::new("private_key", "RPC_URL", config));
    let data = agent.deploy_token(name, uri, symbol, decimals, Some(initial_supply)).await;
    println!("Mint data: {:?}", data);
}
