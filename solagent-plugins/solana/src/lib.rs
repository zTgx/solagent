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

mod close_empty_token_accounts;
pub use close_empty_token_accounts::{close_empty_token_accounts, CloseEmptyTokenAccountsData};

mod get_balance;
pub use get_balance::get_balance;

mod request_faucet_funds;
pub use request_faucet_funds::request_faucet_funds;

mod get_tps;
pub use get_tps::get_tps;

mod transfer;
pub use transfer::transfer;

mod deploy_token;
pub use deploy_token::deploy_token;

mod deploy_collection;
pub use deploy_collection::deploy_collection;

mod get_balance_other;
pub use get_balance_other::get_balance_other;

mod get_wallet_address;
pub use get_wallet_address::get_wallet_address;

mod mint_nft;
pub use mint_nft::mint_nft_to_collection;

use mpl_token_metadata::types::Creator;
use serde::{Deserialize, Serialize};
use solagent_core::solana_sdk::pubkey::Pubkey;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeployedData {
    pub mint: String,      // mint address
    pub signature: String, // Tx hash
}

impl DeployedData {
    pub fn new(mint: String, signature: String) -> Self {
        DeployedData { mint, signature }
    }
}

/// Metadata for deploying an NFT/Collection.
///
/// # Fields
///
/// - `name`: The name of the NFT.
/// - `uri`: The URI for the collection's metadata.
/// - `basis_points`: Optional. The basis points for the NFT.
/// - `creators`: Optional. A list of creators associated with the NFT.
///
// #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct NFTMetadata {
    pub name: String,
    pub uri: String,
    pub basis_points: Option<u16>,      // Optional basis points
    pub creators: Option<Vec<Creator>>, // Optional list of creators
}

impl NFTMetadata {
    pub fn new(name: &str, uri: &str, basis_points: Option<u16>, creators: Option<Vec<(Pubkey, u8)>>) -> Self {
        let creators = creators.map(|creator_tuples| {
            creator_tuples
                .into_iter()
                .map(|(pubkey, share)| Creator { address: pubkey, verified: true, share })
                .collect::<Vec<Creator>>()
        });

        NFTMetadata { name: name.to_string(), uri: uri.to_string(), basis_points, creators }
    }
}
