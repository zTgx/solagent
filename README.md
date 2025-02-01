<div align="center">

# solagent.rs   
  [<img alt="crates.io" src="https://img.shields.io/crates/v/solagent?style=for-the-badge&logo=rust">](https://crates.io/crates/solagent)
  [<img alt="docs.rs" src="https://img.shields.io/docsrs/solagent?style=for-the-badge&logo=docs.rs">](https://docs.rs/solagent)
  [<img alt="crates.io" src="https://img.shields.io/crates/d/solagent?style=for-the-badge&logo=rust">](https://crates.io/crates/solagent)
</div>

</br>

`solagent.rs` - An open-source Rust library for connecting AI agents to Solana protocols. 

## Quick Start
```rust
use solagent::{create_solana_tools, Config, SolanaAgentKit};

#[tokio::main]
async fn main() {
    let config = Config {
        openai_api_key: Some("your_api_key".to_string()),
        ..Default::default()
    };
    let agent = SolanaAgentKit::new("private_key", "RPC_URL", config);
    let toolset = create_solana_tools(agent);
}
```

## Packages
### Core
|  | crates.io package |
| --- | --- |
| Core | [solagent-core](https://crates.io/crates/solagent-core) |

### Agent Framework Adapters
| Adapter | crates.io package |
| --- | --- |
| rig | [solagent-adapter-rig]() | 
| rig | [solagent-adapter-rig]() | 

### Plugins
| Plugin | Tools | crates.io package |
| --- | --- | --- |
| gibwork | Create a task on Gibwork | [solagent-plugin-gibwork](https://crates.io/crates/solagent-plugin-gibwork) |
 

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

## Support Us
Thank you for your support of solagent.rs! Your donations will help us maintain and develop this project better.  
Donation Addresses:  
**Solana: qsvR7V3DtbbxAEs4TkGiQL9N9Z1GccfV6Q6Vm2oHViS**  
**Ethereum: 0x972b8d20f5847B03Df43d9A595B7e83A7bbCD951**  

## Our Supporters  
SendAI: [solscan.io](https://solscan.io/tx/nf3B1zaTZcLuCLVTkLFHuTqjVjLUwXHkCnN3Tdm7PHSDunjJD6tZHYHgijJKbCcchHaxVYWM4uEgieQyLjRBCR4)  

</br>
Thank you to all our generous supporters!  
