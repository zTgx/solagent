# Rig Adapter for solagent

## Installation
```toml
[dependencies]
# add wallet
solagent-wallet-solana = "0.1.3"

# add core
solagent-core = "0.1.6"

# add plugin
solagent-plugin-birdeye = "0.1.7"
```

## Usage

```rust
use solagent_core::rig::{completion::Prompt, providers};
use solagent_rig_birdeye::MarketData;
use std::sync::Arc;

fn setup_solagent() -> SolanaAgentKit {
    let keypair = Keypair::new();
    let private_key = keypair.to_base58_string();
    env::set_var("TEST_PRIVATE_KEY", &private_key);

    let wallet = Wallet::from_env("TEST_PRIVATE_KEY").unwrap();

    let birdeye_api_key =
            env::var("BIRDEYE_API_KEY").unwrap();

    let config = ConfigBuilder::default().birdeye_api_key(birdeye_api_key).build();
    let agent = SolanaAgentKit::new(wallet, "https://api.devnet.solana.com", config);

    agent
}

#[tokio::main]
async fn main() {
    // Setup SolanaAgentKit
    let agent_kit = setup_solagent();

    // Adapater
    let md = MarketData::new(Arc::new(agent_kit));

    let token_id = "So11111111111111111111111111111111111111112";
    let prompt = format!("Get market data of single token by birdeye api: {}", token_id);

    let client = providers::openai::Client::from_url("ollama", "http://localhost:11434/v1");
    let agent = client
        .agent("llama3.2")
        .preamble(
            "You are an assistant here to help the user select which tool is most appropriate to perform operations.",
        )
        .tool(md)
        .build();

    // Prompt the agent and print the response
    let response = agent.prompt(&prompt).await.unwrap();
    println!("{}", response);
}
```