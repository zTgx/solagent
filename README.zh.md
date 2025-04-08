# solagent.rs

[![License](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.83%2B-orange)](https://www.rust-lang.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0%2B-blue)](https://www.typescriptlang.org)

> 专为 Solana DeFAI 打造的轻量级智能代理工具包

## 🚀 核心特性
🎯 **专注 Solana** - 为 Solana DeFAI 操作优化  
🪶 **轻量级** - 仅核心必要功能，无冗余  
🔌 **插件架构** - 核心+插件模块化设计  
🛠️ **可扩展** - 轻松创建自定义插件/脚本  
🌐 **多语言** - 原生 Rust + TypeScript 支持  

### 为什么支持双语言?
| Rust | TypeScript |
|------|------------|
| ✅ 原生 Solana 程序交互 | ✅ Web3/DApp 集成 |
| ✅ 高性能底层 | ✅ 浏览器兼容 |
| ✅ 安全关键操作 | ✅ 快速原型开发 |


## 🛠️ 使用示例

### >>> Rust
---
```toml
[dependencies]
solagent = { version = "0.3", features = ["core"] }
solagent-plugins = { version = "0.3", features = ["swap"] }  # 可选插件
```

### Rust (后端服务)
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

### >>> TypeScript
---

```bash
npm install @solagent/core @solagent/swap-plugin  # 模块化安装
```

### TypeScript (浏览器/DApp)
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

## 📚 文档

- [API 文档](https://docs.solagent.rs)
- [插件开发指南](./docs/PLUGINS.md)
- [安全实践](./docs/SECURITY.md)

## 🤝 参与贡献
详见 [CONTRIBUTING.md](./CONTRIBUTING.md):
- 插件开发规范
- 测试要求
- PR 审核流程

---
[README Documentation](./README.md)