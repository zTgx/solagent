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

use solagent_core::{
    solana_sdk::{pubkey::Pubkey, signature::Keypair},
    Config, SolanaAgentKit,
};
use solagent_plugin_solana::{deploy_collection, NFTMetadata};

/// Example on devnet
/// Mint: HHV3DX4UT4u3vBek2XCaZeAyox88zuhWfcLRJbFx1oYt

#[tokio::main]
async fn main() {
    // Create a new keypair
    let keypair = Keypair::new();
    // Encode the secret key to base58
    let private_key = keypair.to_base58_string();

    let config = Config { cookie_api_key: Some("".to_string()), ..Default::default() };
    let agent = SolanaAgentKit::new(&private_key, "https://api.devnet.solana.com", config);

    let name = "Solagent Collection";
    let uri = "uri";
    let royalty_basis_points = Some(500);
    let creators = vec![(Pubkey::from_str_const("pubkey"), 100)];
    let options = NFTMetadata::new(name, uri, royalty_basis_points, Some(creators));

    let data = deploy_collection(&agent, &options).await.unwrap();
    println!("Deploy Data: {:?}", data);
}
