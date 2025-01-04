use solagent::SOL_AGENT;

#[tokio::main]
async fn main() {
    let name = "Solagent".to_string();
    let uri = "solagent.rs".to_string();
    let symbol = "SOLA".to_string();
    let decimals = 1;
    let initial_supply = 1_000_000_000_u64;
    let tx = SOL_AGENT
        .deploy_token(name, uri, symbol, decimals, Some(initial_supply))
        .await;
    println!(">>> deploy tx: {:?}", tx);
    // 3kvSrsPwtYi6RkWymJocQcezwiDpqMfDjWazYAaibDmY
}
