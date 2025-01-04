use mpl_token_metadata::types::Creator;
use solagent::{config::CONFIG, token::CollectionOptions, wallet::Wallet, SOL_AGENT};

#[tokio::main]
async fn main() {
    let wallet_path = &CONFIG.agent.wallet_path;
    let wallet = Wallet::load(wallet_path);

    let options = CollectionOptions {
        name: "Solagent Collection".to_string(),
        uri: "https://solagent.rs".to_string(),
        royalty_basis_points: Some(500),
        creators: Some(vec![Creator {
            address: wallet.address,
            verified: true,
            share: 100,
        }]),
    };

    let tx = SOL_AGENT.deploy_collection(options).await;
    println!(">>> deploy collection tx: {:?}", tx);
    // 3kvSrsPwtYi6RkWymJocQcezwiDpqMfDjWazYAaibDmY
}
