use std::sync::Arc;

use solagent::{create_solana_tools, SolAgent};

#[tokio::main]
async fn main() {
    let agent = Arc::new(SolAgent::new(
        "private_key",
        "https://api.devnet.solana.com",
        "openai_api_key",
    ));
    let _tools = create_solana_tools(agent);
}
