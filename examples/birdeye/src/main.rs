use solagent_core::{ConfigBuilder, SolanaAgentKit};
use solagent_plugin_birdeye::get_token_metadata;
use solagent_wallet_solana::Wallet;

#[tokio::main]
async fn main() {
    let wallet = Wallet::from_env("variable_name").unwrap();

    let config = ConfigBuilder::default().birdeye_api_key("".into()).build();
    let agent = SolanaAgentKit::new(wallet, "https://api.devnet.solana.com", config);

    let data = get_token_metadata(&agent, "So11111111111111111111111111111111111111112")
        .await
        .unwrap();
    println!("{:#?}", data);
}
