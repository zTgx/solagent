use solagent::SolAgent;

#[tokio::main]
async fn main() {
    let price = SolAgent::fetch_price("So11111111111111111111111111111111111111112")
        .await
        .unwrap();
    println!("Price: {}", price);
}
