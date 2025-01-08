use solagent::SolAgent;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let agent = Arc::new(SolAgent::new(
        "private_key",
        "https://api.devnet.solana.com",
        "openai_api_key",
    ));
    let balance = agent.get_balance(None).await.unwrap();
    println!("My balance: {}", balance);
}
