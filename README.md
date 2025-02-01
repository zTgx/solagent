<div align="center">

# solagent.rs   
  [<img alt="crates.io" src="https://img.shields.io/crates/v/solagent?style=for-the-badge&logo=rust">](https://crates.io/crates/solagent)
  [<img alt="docs.rs" src="https://img.shields.io/docsrs/solagent?style=for-the-badge&logo=docs.rs">](https://docs.rs/solagent)
  [<img alt="crates.io" src="https://img.shields.io/crates/d/solagent?style=for-the-badge&logo=rust">](https://crates.io/crates/solagent)
</div>

</br>

`solagent.rs` - An open-source Rust library for connecting AI agents to Solana protocols. 

## Usage
```toml
[dependencies]
solagent-core = "0.1.0"
solagent-plugin-solana = "0.1.0"
solagent-rig-goplus = "0.1.0"
```
```rust
use solagent_core::{
    rig::{
        completion::Prompt,
        providers::gemini::{self, completion::GEMINI_1_5_FLASH},
    },
    solana_sdk::signer::keypair::Keypair,
    *,
};

#[tokio::main]
async fn main() {
    // Create a new keypair
    let keypair = Keypair::new();
    // Encode the secret key to base58
    let private_key = keypair.to_base58_string();

    let config = Config { openai_api_key: Some("your_api_key".to_string()), ..Default::default() };
    let agent = SolanaAgentKit::new(&private_key, "https://api.devnet.solana.com", config);

    let _v =
        solagent_plugin_goplus::get_solana_token_security_info("So11111111111111111111111111111111111111112").await;

    let _v = solagent_plugin_solana::get_tps(&agent).await;

    let tool = solagent_rig_goplus::TokenMaliciousInfo::new();

    let client = gemini::Client::from_env();
    let agent = client
        .agent(GEMINI_1_5_FLASH)
        .preamble(
            "You are an assistant here to help the user select which tool is most appropriate to perform operations.",
        )
        .max_tokens(1024)
        .tool(tool)
        .build();

    let response = agent
        .prompt("check token malicious solana So11111111111111111111111111111111111111112")
        .await
        .expect("Failed to prompt Gemini");

    println!("Gemini response: {response}");
}
```

## Packages
### Core
|  | crates.io package |
| --- | --- |
| Core | [solagent-core](https://crates.io/crates/solagent-core) |

### Agent Framework Adapters
| Adapter | package |
| --- | --- |
| rig | [solagent-adapter-rig](./solagent-adapters/rig/) | 


### Plugins
| Plugin | Tools | crates.io package |
| --- | --- | --- |
| gibwork | Create a task on Gibwork | [solagent-plugin-gibwork](https://crates.io/crates/solagent-plugin-gibwork) |
| goplus | Token Security API | [solagent-plugin-goplus](https://crates.io/crates/solagent-plugin-goplus) |
| helius | Webhook operates  | [solagent-plugin-helius](https://crates.io/crates/solagent-plugin-helius) |
| jupiter | Jupiter Exchange  | [solagent-plugin-jupiter](https://crates.io/crates/solagent-plugin-jupiter) |
| pumpfun | Launch Token on pumpfun  | [solagent-plugin-pumpfun](https://crates.io/crates/solagent-plugin-pumpfun) |
| pyth | Fetch price from Pyth  | [solagent-plugin-pyth](https://crates.io/crates/solagent-plugin-pyth) |
| rugcheck | Rug check | [solagent-plugin-rugcheck](https://crates.io/crates/solagent-plugin-rugcheck) |
| solana | Operations on solana | [solagent-plugin-solana](https://crates.io/crates/solagent-plugin-solana) |
| solayer | Stake with solayer | [solagent-plugin-solayer](https://crates.io/crates/solagent-plugin-solayer) |
 

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

