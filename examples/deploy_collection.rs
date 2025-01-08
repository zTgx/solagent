use mpl_token_metadata::types::Creator;
use solagent::{CollectionOptions, SolAgent};
use solana_sdk::pubkey::Pubkey;

#[tokio::main]
async fn main() {
    let options = CollectionOptions {
        name: "Solagent Collection".to_string(),
        uri: "https://img.zol.com.cn/group/217/a2167467.jpg".to_string(),
        royalty_basis_points: Some(500),
        creators: Some(vec![Creator {
            address: Pubkey::from_str_const("HHV3DX4UT4u3vBek2XCaZeAyox88zuhWfcLRJbFx1oYt"),
            verified: true,
            share: 100,
        }]),
    };

    let agent = SolAgent::new();
    let tx = agent.deploy_collection(options).await;
    println!(">>> deploy collection tx: {:?}", tx);
}
