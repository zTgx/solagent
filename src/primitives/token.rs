use mpl_token_metadata::types::Creator;

/// Options for deploying an NFT collection.
///
/// # Fields
///
/// - `name`: The name of the collection.
/// - `uri`: The URI for the collection's metadata.
/// - `royalty_basis_points`: Optional. The royalty basis points for the collection.
/// - `creators`: Optional. A list of creators associated with the collection.
pub struct CollectionOptions {
    pub name: String,
    pub uri: String,
    pub royalty_basis_points: Option<u16>, // Optional royalty basis points
    pub creators: Option<Vec<Creator>>,    // Optional list of creators
}
