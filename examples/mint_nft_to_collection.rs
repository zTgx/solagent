use solagent::{NftMetadata, SolAgent};
use solana_sdk::pubkey::Pubkey;

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

    let agent = SolAgent::new();
    let deployed_data = agent
        .mint_nft_to_collection(collection, metadata)
        .await
        .unwrap();
    println!("Mint: {}", deployed_data.mint);
    // 5jcsea3EA3kX7mXpy7YvHVFYTDEJeSEXjyicgThnvWUm
    // https://explorer.solana.com/address/5jcsea3EA3kX7mXpy7YvHVFYTDEJeSEXjyicgThnvWUm?cluster=devnet
}
