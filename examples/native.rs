use solagent::SolAgent;

#[tokio::main]
async fn main() {
    // let balance = SOL_AGENT.get_balance(None).await.unwrap();
    // println!("My balance: {}", balance);

    let price_feed_id = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
    let price = SolAgent::fetch_price_by_pyth(price_feed_id).await.unwrap();
    println!("Pyth Price: {}", price);
}
