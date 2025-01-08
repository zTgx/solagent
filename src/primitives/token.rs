use mpl_token_metadata::types::Creator;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

/// Metadata for deploying an NFT/Collection.
///
/// # Fields
///
/// - `name`: The name of the NFT.
/// - `uri`: The URI for the collection's metadata.
/// - `basis_points`: Optional. The basis points for the NFT.
/// - `creators`: Optional. A list of creators associated with the NFT.
pub struct NftMetadata {
    pub(crate) name: String,
    pub(crate) uri: String,
    pub(crate) basis_points: Option<u16>, // Optional basis points
    pub(crate) creators: Option<Vec<Creator>>, // Optional list of creators
}

impl NftMetadata {
    pub fn new(
        name: &str,
        uri: &str,
        basis_points: Option<u16>,
        creators: Option<Vec<(Pubkey, u8)>>,
    ) -> Self {
        let creators = creators.map(|creator_tuples| {
            creator_tuples
                .into_iter()
                .map(|(pubkey, share)| Creator {
                    address: pubkey,
                    verified: true,
                    share,
                })
                .collect::<Vec<Creator>>()
        });

        NftMetadata {
            name: name.to_string(),
            uri: uri.to_string(),
            basis_points,
            creators,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeployedData {
    pub mint: String,      // mint address
    pub signature: String, // Tx hash
}
