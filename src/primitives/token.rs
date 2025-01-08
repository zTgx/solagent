use mpl_token_metadata::types::Creator;
use solana_sdk::pubkey::Pubkey;

/// Options for deploying an NFT collection.
///
/// # Fields
///
/// - `name`: The name of the collection.
/// - `uri`: The URI for the collection's metadata.
/// - `royalty_basis_points`: Optional. The royalty basis points for the collection.
/// - `creators`: Optional. A list of creators associated with the collection.
pub struct CollectionOptions {
    pub(crate) name: String,
    pub(crate) uri: String,
    pub(crate) royalty_basis_points: Option<u16>, // Optional royalty basis points
    pub(crate) creators: Option<Vec<Creator>>,    // Optional list of creators
}

impl CollectionOptions {
    pub fn new(
        name: &str,
        uri: &str,
        royalty_basis_points: Option<u16>,
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

        CollectionOptions {
            name: name.to_string(),
            uri: uri.to_string(),
            royalty_basis_points,
            creators,
        }
    }
}
