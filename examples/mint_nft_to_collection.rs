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

use solagent::{Config, NFTMetadata, SolanaAgentKit};
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;

/// Example on devnet
/// Mint: 5jcsea3EA3kX7mXpy7YvHVFYTDEJeSEXjyicgThnvWUm
/// https://explorer.solana.com/address/5jcsea3EA3kX7mXpy7YvHVFYTDEJeSEXjyicgThnvWUm?cluster=devnet
#[tokio::main]
async fn main() {
    let name = "My First SolanaAgentKit NFT";
    let uri = "uri";
    let royalty_basis_points = Some(500);
    let creators = vec![(Pubkey::from_str_const("pubkey"), 100)];
    let metadata = NFTMetadata::new(name, uri, royalty_basis_points, Some(creators));

    let collection = Pubkey::from_str_const("collection Mint");

    let config = Config { openai_api_key: Some("your_api_key".to_string()), ..Default::default() };
    let agent = Arc::new(SolanaAgentKit::new("private_key", "RPC_URL", config));
    let deployed_data = agent.mint_nft_to_collection(collection, metadata).await.unwrap();
    println!("Mint: {}", deployed_data.mint);
}
