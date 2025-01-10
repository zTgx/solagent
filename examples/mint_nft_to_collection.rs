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
    let name = "My First SolAgent NFT";
    let uri = "https://arweave.net/metadata.json";
    let royalty_basis_points = Some(500);
    let creators = vec![(
        Pubkey::from_str_const("8QB5VckaW3CWv4oZWiMLs1GkdrR5pVcjarAS1U6rG6Wh"),
        100,
    )];
    let metadata = NftMetadata::new(name, uri, royalty_basis_points, Some(creators));

    let collection = Pubkey::from_str_const("HHV3DX4UT4u3vBek2XCaZeAyox88zuhWfcLRJbFx1oYt");

    let agent = Arc::new(SolAgent::new(
        "private_key",
        "https://api.devnet.solana.com",
        "openai_api_key",
    ));
    let deployed_data = agent
        .mint_nft_to_collection(collection, metadata)
        .await
        .unwrap();
    println!("Mint: {}", deployed_data.mint);
    // 5jcsea3EA3kX7mXpy7YvHVFYTDEJeSEXjyicgThnvWUm
    // https://explorer.solana.com/address/5jcsea3EA3kX7mXpy7YvHVFYTDEJeSEXjyicgThnvWUm?cluster=devnet
}
