<div align="center">

# solagent.rs   
  [<img alt="crates.io" src="https://img.shields.io/crates/v/solagent?style=for-the-badge&logo=rust">](https://crates.io/crates/solagent)
  [<img alt="docs.rs" src="https://img.shields.io/docsrs/solagent?style=for-the-badge&logo=docs.rs">](https://docs.rs/solagent)
  [<img alt="crates.io" src="https://img.shields.io/crates/d/solagent?style=for-the-badge&logo=rust">](https://crates.io/crates/solagent)

</div>
An open-source toolkit for connecting AI agents to Solana protocols.

## WIP
Solagent.rs is still in the early stage of development, and its interfaces may change in the future.  
Contributions are welcome! Please feel free to submit a Pull Request.


## 📦 Installation

```bash
[dependencies]
solagent = "0.1.4"
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

### Mint NFT to Collection
```rust
#[tokio::main]
async fn main() {
    let name = "My First SolAgent NFT";
    let uri = "https://arweave.net/metadata.json";
    let royalty_basis_points = Some(500);
    let creators = vec![(
        Pubkey::from_str_const("
            8QB5VckaW3CWv4oZWiMLs1GkdrR5pVcjarAS1U6rG6Wh"),
        100,
    )];
    let metadata = NftMetadata::new(name, uri, royalty_basis_points, Some(creators));

    let collection = Pubkey::from_str_const("
        HHV3DX4UT4u3vBek2XCaZeAyox88zuhWfcLRJbFx1oYt");

    let agent = SolAgent::new();
    let deployed_data = agent
        .mint_nft_to_collection(collection, metadata)
        .await
        .unwrap();
    println!("Mint: {}", deployed_data.mint);
}
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
| Name | Language | Framework | GitHub |
|---|---|---|---|
| **solana-agent-kit** | Typscript | Langchain | [solana-agent-kit](https://github.com/sendaifun/solana-agent-kit) |
| **agentipy** | Python | Langchain, pydantic | [agentipy](https://github.com/niceberginc/agentipy) |
| **solagent.rs** | Rust | rig | [solagent.rs](https://github.com/zTgx/solagent.rs) |

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=solagent/solagent&type=Date)](https://star-history.com/#solagent/solagent&Date)

## License

MIT License

## Security

This toolkit handles private keys and transactions. Always ensure you're using it in a secure environment and never share your private keys.
