<div align="center">

[Docs](https://docs.solagent.rs) | [X](https://x.com/ztgx5) | [Telegram](https://t.me/solagent_rs)

# solagent.rs   
An open-source Agent framework for connecting any AI agents to Solana protocols in Rust. 

![Crates Downloads](https://img.shields.io/crates/d/solagent-core?logo=rust)
![GitHub License](https://img.shields.io/github/license/zTgx/solagent.rs)

</div>

</br>

# SOLAGENT.RS
![X (formerly Twitter) Follow](https://img.shields.io/twitter/follow/ztgx5)

* **[Agent Frameworks]()**: Rig
* **[Wallets]()**: keypair, solana
* **[+50 tools]()**: Birdeye, dexscreener, solana, jupiter, helius nd more
* **[Chains]()**: Solana

## Quick start
* Add dependencies
```toml
[dependencies]
# add wallet
solagent-wallet-solana = "0.1.3"

# add core
solagent-core = "0.1.6"

# add plugin
solagent-plugin-birdeye = "0.1.7"
```
* Create agent
```rust
use solagent_core::{ConfigBuilder, SolanaAgentKit};
use solagent_plugin_birdeye::get_token_metadata;
use solagent_wallet_solana::Wallet;

#[tokio::main]
async fn main() {
    let wallet = Wallet::from_env("SOLANA_WALLET").unwrap();
    let config = ConfigBuilder::default().birdeye_api_key("api_key".into()).build();

    let agent = SolanaAgentKit::new(wallet, "https://api.devnet.solana.com", config);
    let data = get_token_metadata(&agent, "So11111111111111111111111111111111111111112")
        .await
        .unwrap();
    println!("{:#?}", data);
}
```
## Table of Contens
* [How-To-Use](./docs/how-to-usage.md)
* [How-To-Add-NewFeature](./docs/hot-to-add-feature.md)


## Packages
### Core
| Crate | Package | Version | 
| --- | --- | --- |
| Core | [solagent-core](https://crates.io/crates/solagent-core) | ![Version](https://img.shields.io/crates/v/solagent-core) |
| Wallet | [solagent-wallet-solana](https://crates.io/crates/solagent-wallet-solana) | ![Version](https://img.shields.io/crates/v/solagent-wallet-solana) |

### Tools
| Tools | Description | Package | Version |
| --- | --- | --- | --- | 
| gibwork | Create a task on Gibwork | [solagent-rig-gibwork](https://crates.io/crates/solagent-rig-gibwork) | ![Version](https://img.shields.io/crates/v/solagent-rig-gibwork) |
| goplus | Token Security API | [solagent-rig-goplus](https://crates.io/crates/solagent-rig-goplus) | ![Version](https://img.shields.io/crates/v/solagent-rig-goplus) |
| helius | Webhook operates  | [solagent-rig-helius](https://crates.io/crates/solagent-rig-helius) | ![Version](https://img.shields.io/crates/v/solagent-rig-helius) |
| jupiter | Jupiter Exchange  | [solagent-rig-jupiter](https://crates.io/crates/solagent-rig-jupiter) | ![Version](https://img.shields.io/crates/v/solagent-rig-jupiter) |
| pumpfun | Launch Token on pumpfun  | [solagent-rig-pumpfun](https://crates.io/crates/solagent-rig-pumpfun) | ![Version](https://img.shields.io/crates/v/solagent-rig-pumpfun) |
| pyth | Fetch price from Pyth  | [solagent-rig-pyth](https://crates.io/crates/solagent-rig-pyth) | ![Version](https://img.shields.io/crates/v/solagent-rig-pyth) |
| rugcheck | Rug check | [solagent-rig-rugcheck](https://crates.io/crates/solagent-rig-rugcheck) | ![Version](https://img.shields.io/crates/v/solagent-rig-rugcheck) |
| solana | Operations on solana | [solagent-rig-solana](https://crates.io/crates/solagent-rig-solana) | ![Version](https://img.shields.io/crates/v/solagent-rig-solana) |
| solayer | Stake with solayer | [solagent-rig-solayer](https://crates.io/crates/solagent-rig-solayer) | ![Version](https://img.shields.io/crates/v/solagent-rig-solayer) |
| cookie | Cookie Data Swarm APIs | [solagent-rig-cookie](https://crates.io/crates/solagent-rig-cookie) | ![Version](https://img.shields.io/crates/v/solagent-rig-cookie) |
| dexscreener | Dexscreener data APIs | [solagent-rig-dexscreener](https://crates.io/crates/solagent-rig-dexscreener) | ![Version](https://img.shields.io/crates/v/solagent-rig-dexscreener) |
| birdeye | Birdeye API | [solagent-rig-birdeye](https://crates.io/crates/solagent-rig-birdeye) | ![Version](https://img.shields.io/crates/v/solagent-rig-birdeye) |
 

### Plugins
| Plugin | Description | Package | Version |
| --- | --- | --- | --- | 
| gibwork | Create a task on Gibwork | [solagent-plugin-gibwork](https://crates.io/crates/solagent-plugin-gibwork) | ![Version](https://img.shields.io/crates/v/solagent-plugin-gibwork) |
| goplus | Token Security API | [solagent-plugin-goplus](https://crates.io/crates/solagent-plugin-goplus) | ![Version](https://img.shields.io/crates/v/solagent-plugin-goplus) |
| helius | Webhook operates  | [solagent-plugin-helius](https://crates.io/crates/solagent-plugin-helius) | ![Version](https://img.shields.io/crates/v/solagent-plugin-helius) |
| jupiter | Jupiter Exchange  | [solagent-plugin-jupiter](https://crates.io/crates/solagent-plugin-jupiter) | ![Version](https://img.shields.io/crates/v/solagent-plugin-jupiter) |
| pumpfun | Launch Token on pumpfun  | [solagent-plugin-pumpfun](https://crates.io/crates/solagent-plugin-pumpfun) | ![Version](https://img.shields.io/crates/v/solagent-plugin-pumpfun) |
| pyth | Fetch price from Pyth  | [solagent-plugin-pyth](https://crates.io/crates/solagent-plugin-pyth) | ![Version](https://img.shields.io/crates/v/solagent-plugin-pyth) |
| rugcheck | Rug check | [solagent-plugin-rugcheck](https://crates.io/crates/solagent-plugin-rugcheck) | ![Version](https://img.shields.io/crates/v/solagent-plugin-rugcheck) |
| solana | Operations on solana | [solagent-plugin-solana](https://crates.io/crates/solagent-plugin-solana) | ![Version](https://img.shields.io/crates/v/solagent-plugin-solana) |
| solayer | Stake with solayer | [solagent-plugin-solayer](https://crates.io/crates/solagent-plugin-solayer) | ![Version](https://img.shields.io/crates/v/solagent-plugin-solayer) |
| cookie | Cookie Data Swarm APIs | [solagent-plugin-cookie](https://crates.io/crates/solagent-plugin-cookie) | ![Version](https://img.shields.io/crates/v/solagent-plugin-cookie) |
| dexscreener | Dexscreener data APIs | [solagent-plugin-dexscreener](https://crates.io/crates/solagent-plugin-dexscreener) | ![Version](https://img.shields.io/crates/v/solagent-plugin-dexscreener) |
| birdeye | Birdeye API | [solagent-plugin-birdeye](https://crates.io/crates/solagent-plugin-birdeye) | ![Version](https://img.shields.io/crates/v/solagent-plugin-birdeye) |
 

## Contributors

<a href="https://github.com/zTgx/solagent.rs/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=zTgx/solagent.rs" />
</a>

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=zTgx/solagent.rs&type=Date)](https://star-history.com/#zTgx/solagent.rs&Date)

## Support solagent.rs
Thank you for your support of solagent.rs!   
Donation Addresses:  
**Solana: qsvR7V3DtbbxAEs4TkGiQL9N9Z1GccfV6Q6Vm2oHViS**  
**Ethereum: 0x972b8d20f5847B03Df43d9A595B7e83A7bbCD951**  

## Supporters  
SendAI: [solscan.io](https://solscan.io/tx/nf3B1zaTZcLuCLVTkLFHuTqjVjLUwXHkCnN3Tdm7PHSDunjJD6tZHYHgijJKbCcchHaxVYWM4uEgieQyLjRBCR4)  

