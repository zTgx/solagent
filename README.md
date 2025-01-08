# solagent.rs   
  [<img alt="crates.io" src="https://img.shields.io/crates/v/solagent?style=for-the-badge&logo=rust">](https://crates.io/crates/solagent)
  [<img alt="docs.rs" src="https://img.shields.io/docsrs/solagent?style=for-the-badge&logo=docs.rs">](https://docs.rs/solagent)
  [<img alt="crates.io" src="https://img.shields.io/crates/d/solagent?style=for-the-badge&logo=rust">](https://crates.io/crates/solagent)
  
connect any Ai agents to solana protocols in Rust.

## 📦 Installation

```bash
[dependencies]
solagent = "0.1.3"
```

## Quick Start
```shell
cp exampel.config.toml config.toml
```
```rust
use solagent::create_solana_tools;

#[tokio::main]
async fn main() {
    let tools = create_solana_tools();
}
```
## Or running it locally
```rust
use rig::{completion::Prompt, providers};
use solagent::fetch_price::FetchPrice;

#[tokio::main]
async fn main() -> Result<(), String> {
    let token_id = "So11111111111111111111111111111111111111112";
    let prompt = format!("fetch price of token_id {}", token_id);

    let client = providers::openai::Client::from_url("ollama", "http://localhost:11434/v1");
    let comedian_agent = client
        .agent("llama3.2")
        .preamble(r#"
You are an assistant here to help the user select
which tool is most appropriate to perform operations.
        "#)
        .tool(FetchPrice)
        .build();

    let response = comedian_agent.prompt(&prompt).await.unwrap();
    println!("{}", response);

    Ok(())
}
```

## Usage Examples
### Deploy a New Token
```rust
    let name = "my ai token".to_string();
    let uri = "uri".to_string();
    let symbol = "SOLA".to_string();
    let decimals = 9;
    let initial_supply = 1_000_000_000_u64;

    let agent = SolAgent::new();
    let mint_pubkey = agent
      .deploy_token(name, uri, symbol, decimals, Some(initial_supply)).await;
    println!("Token Mint Address: {:?}", mint_pubkey);
```

### Create NFT Collection
```rust
    let wallet_path = &CONFIG.agent.wallet_path;
    let wallet = Wallet::load(wallet_path);

    let options = CollectionOptions {
        name: "Solagent Collection".to_string(),
        uri: "https://arweave.net/metadata.json".to_string(),
        royalty_basis_points: Some(500),
        creators: Some(vec![Creator {
            address: wallet.address,
            verified: true,
            share: 100,
        }]),
    };

    let agent = SolAgent::new();
    let _tx = agent.deploy_collection(options).await;
```

### Fetch Price Data from Pyth
```rust
    let agent = SolAgent::new();
    let price_feed_id = agent.fetch_pyth_price_feed_id("SOL")
        .await
        .expect("fetch_pyth_price_feed_id");
    let price = agent.fetch_price_by_pyth(&price_feed_id)
        .await
        .expect("fetch_price_by_pyth");
    println!("Price of SOL/USD: {}", price)
```

## More examples
More examples can be found in the [examples](examples/).  

## SDKs
### You can use API wrappers in your favorite language.
Typscript - [solana-agent-kit](https://github.com/sendaifun/solana-agent-kit)  
Python - [agentipy](https://github.com/niceberginc/agentipy)  
Rust - [solagent.rs](https://github.com/zTgx/solagent.rs)
