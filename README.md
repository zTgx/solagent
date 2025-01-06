# solagent.rs   
  [<img alt="crates.io" src="https://img.shields.io/crates/v/solagent?style=for-the-badge&logo=rust">](https://crates.io/crates/solagent)
  [<img alt="docs.rs" src="https://img.shields.io/docsrs/solagent?style=for-the-badge&logo=docs.rs">](https://docs.rs/solagent)
  [<img alt="crates.io" src="https://img.shields.io/crates/d/solagent?style=for-the-badge&logo=rust">](https://crates.io/crates/solagent)
  
### WIP
connect any Ai agents to solana protocols in Rust.

## 📦 Installation

```bash
[dependencies]
solagent = "0.1.2"
```

## Quick Start
```shell
cp exampel.config.toml config.toml
```
```rust
use solagent::{toolset::create_solana_tools, SOL_AGENT};

#[tokio::main]
async fn main() {
    let tools = create_solana_tools(&SOL_AGENT);
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
    let mint_pubkey = SOL_AGENT.deploy_token(name, uri, symbol, decimals, Some(initial_supply)).await;
    println!("Token Mint Address: {:?}", mint_pubkey);
```

## More examples
More examples can be found in the [examples](examples/).  

