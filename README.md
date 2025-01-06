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
use solagent::SolAgent;

#[tokio::main]
async fn main() {
    let price_feed_id = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
    let price = SolAgent::fetch_price_by_pyth(price_feed_id).await.unwrap();
    println!("Pyth Price: {}", price);
}
```

## More examples
More examples can be found in the [examples](examples/).  

