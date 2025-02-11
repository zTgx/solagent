use solagent_core::{IWallet, SolanaAgentKit};
use std::error::Error;

mod primitive;
pub use primitive::*;

pub async fn get_token_overview<W: IWallet>(
    agent: &SolanaAgentKit<W>,
    address: &str,
) -> Result<Response, Box<dyn Error>> {
    let api_key =
        agent.config.birdeye_api_key.as_ref().ok_or("Missing Birdeye API key in agent.config.birdeye_api_key")?;

    let client = reqwest::Client::new();
    let url = format!("{}/defi/token_overview", BIRDEYE_URL);

    let resp = client
        .get(url)
        .query(&[("address", address)])
        .header("X-API-KEY", api_key)
        .header("accept", "application/json")
        .header("x-chain", "solana")
        .send()
        .await?;

    Ok(resp.json::<Response>().await?)
}
