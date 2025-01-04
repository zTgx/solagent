use solagent::SOL_AGENT;

#[tokio::main]
async fn main() {
    let balance = SOL_AGENT.get_balance(None).await.unwrap();
    println!("My balance: {}", balance);
}
