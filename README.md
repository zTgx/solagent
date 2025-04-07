
# solagent.rs

[![License](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.83%2B-orange)](https://www.rust-lang.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0%2B-blue)](https://www.typescriptlang.org)

> The lightweight, plugin-oriented toolkit for building DeFi agents on Solana

## 🌟 Core Features

🎯 **Solana-Focused** - Optimized for Solana DeFi operations  
🪶 **Lightweight** - Essential core only, zero bloat  
🔌 **Plugin Architecture** - Core + Plugin modular design  
🛠️ **Extensible** - Easily create custom plugins/scripts  
🌐 **Multi-Language** - Native Rust + TypeScript support  

### Why Dual Language?
| Rust | TypeScript |
|------|------------|
| ✅ Native Solana program interaction | ✅ Web3/DApp integration |
| ✅ High-performance base layer | ✅ Browser compatible |
| ✅ Security-critical operations | ✅ Rapid prototyping |


## 🛠️ Usage Examples
### >>> Rust
--- 
```toml
[dependencies]
solagent = { version = "0.3", features = ["core"] }
solagent-plugins = { version = "0.3", features = ["swap"] }  # Optional plugins
```
### Rust (Backend Services)
```rust
use solagent::prelude::*;

let agent = SolAgent::new()
    .with_plugin(OrcaSwapPlugin::default())
    .with_plugin(PortLendingPlugin::new(lending_config));

let tx = agent.swap(SwapParams {
    input_token: Token::SOL,
    amount: 1.0,
    output_token: Token::USDC
}).await?;
```

### TypeScript
```bash
npm install @solagent/core @solagent/swap-plugin  # Modular packages
```
### TypeScript (Browser/DApps)
```typescript
import { SolAgent } from '@solagent/core';
import { RaydiumPlugin } from '@solagent/raydium-plugin';

const agent = new SolAgent()
  .use(new RaydiumPlugin({ version: 'v4' }));

const { txId } = await agent.swap({
  inputMint: 'So111...',
  amount: 1,
  outputMint: 'EPjFW...'
});
```

## 📚 Documentation

- [API Reference](https://docs.solagent.rs)
- [Plugin Development Guide](./docs/PLUGINS.md)
- [Security Best Practices](./docs/SECURITY.md)

## 🤝 Contributing
See [CONTRIBUTING.md](./CONTRIBUTING.md) for:
- Plugin development standards
- Testing requirements
- PR review process

[中文文档](./README.zh.md)
