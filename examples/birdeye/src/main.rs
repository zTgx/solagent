use solagent_core::{ConfigBuilder, SolanaAgentKit, solana_sdk::signature::Keypair};
use solagent_plugin_birdeye::{get_token_holders, TokenHolderQueryParams};
use solagent_wallet_solana::Wallet;
use std::env;

#[tokio::main]
async fn main() {
    let keypair = Keypair::new();
    let private_key = keypair.to_base58_string();
    env::set_var("TEST_PRIVATE_KEY", &private_key);

    let wallet = Wallet::from_env("TEST_PRIVATE_KEY").unwrap();

    let birdeye_api_key =
            env::var("BIRDEYE_API_KEY").unwrap();

    let config = ConfigBuilder::default().birdeye_api_key(birdeye_api_key).build();
    let agent = SolanaAgentKit::new(wallet, "https://api.devnet.solana.com", config);

    let address = "So11111111111111111111111111111111111111112";
    let query = TokenHolderQueryParams::new(address.to_string(), None, None);
    let overview = get_token_holders(&agent, query).await.unwrap();
    println!("{:#?}", overview);



}
