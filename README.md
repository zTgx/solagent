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
```rust
use std::sync::Arc;
use solagent::create_solana_tools;

#[tokio::main]
async fn main() {
    let agent = Arc::new(SolAgent::new("private_key_bs58", "https://api.devnet.solana.com", "openai_api_key"));
    let toolset = create_solana_tools(agent);
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

    let agent = Arc::new(SolAgent::new("private_key_bs58", "https://api.devnet.solana.com", "openai_api_key"));
    let mint_pubkey = agent
      .deploy_token(name, uri, symbol, decimals, Some(initial_supply)).await;
    println!("Token Mint Address: {:?}", mint_pubkey);
```

### Create NFT Collection
```rust
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

    let agent = Arc::new(SolAgent::new("private_key_bs58", "https://api.devnet.solana.com", "openai_api_key"));
    let _tx = agent.deploy_collection(options).await;
```

### Mint NFT to Collection
```rust
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

    let agent = Arc::new(SolAgent::new("private_key_bs58", "https://api.devnet.solana.com", "openai_api_key"));
    let deployed_data = agent
        .mint_nft_to_collection(collection, metadata)
        .await
        .unwrap();
    println!("Mint: {}", deployed_data.mint);
}
```

### Fetch Price Data from Pyth
```rust
    let agent = Arc::new(SolAgent::new("private_key_bs58", "https://api.devnet.solana.com", "openai_api_key"));
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

## 
## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Contributors

<a href="https://github.com/zTgx/solagent.rs/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=zTgx/solagent.rs" />
</a>

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=zTgx/solagent.rs&type=Date)](https://star-history.com/#zTgx/solagent.rs&Date)

## License

Apache-2 License

## Security

This toolkit handles private keys and transactions. Always ensure you're using it in a secure environment and never share your private keys.
