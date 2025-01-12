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

use solagent::{NftMetadata, SolAgent};
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let name = "Solagent Collection";
    let uri = "https://arweave.net/metadata.json";
    let royalty_basis_points = Some(500);
    let creators = vec![(Pubkey::from_str_const("HHV3DX4UT4u3vBek2XCaZeAyox88zuhWfcLRJbFx1oYt"), 100)];
    let options = NftMetadata::new(name, uri, royalty_basis_points, Some(creators));

    let agent = Arc::new(SolAgent::new("private_key", "https://api.devnet.solana.com", "openai_api_key"));
    let tx = agent.deploy_collection(options).await.unwrap();
    println!("Mint: {:?}", tx.0);
}
