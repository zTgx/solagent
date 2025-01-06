use solagent::{toolset::create_solana_tools, SOL_AGENT};

#[tokio::main]
async fn main() {
    let _tools = create_solana_tools(&SOL_AGENT);
}
