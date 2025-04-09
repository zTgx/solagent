# @solagent/core

[![npm version](https://img.shields.io/npm/v/@solagent/core.svg)](https://www.npmjs.com/package/@solagent/core)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Core utilities for SolAgent ecosystem, providing seamless integration with Solana blockchain.

## Features

- 🏗️ Solana agent management with configurable options
- 🔗 Built-in RPC connection handling
- 💰 Balance checking and account utilities
- ⚙️ Customizable configuration for various services

## Installation

```bash
npm install @solagent/core
# or
yarn add @solagent/core
```

## Usage

### Basic Initialization

```typescript
import { Wallet } from '@solagent/wallet';
import { SolanaAgentKit } from '@solagent/core';
import { Connection } from '@solana/web3.js';

// Initialize with Connection object
const wallet = new Wallet();
const connection = new Connection('https://api.mainnet-beta.solana.com');
const agent = new SolanaAgentKit(wallet, connection);

// Or use the factory method
const agent = SolanaAgentKit.create(
  wallet,
  'https://api.mainnet-beta.solana.com'
);
```

### With Custom Configuration

```typescript
const config = {
  heliusApiKey: 'your_api_key_here',
  jupiterFeeBps: 50, // 0.5% fee
  // ...other config options
};

const agent = new SolanaAgentKit(wallet, connection, config);
```

### Checking Balance

```typescript
const balance = await agent.getBalance();
console.log(`Balance: ${balance} lamports`);
```

## API Reference

### `SolanaAgentKit`

#### Constructor
```typescript
new SolanaAgentKit(
  wallet: Wallet,
  connection: Connection,
  config?: Config
)
```

#### Static Method
```typescript
static create(
  wallet: Wallet,
  rpcUrl: string,
  config?: Config
): SolanaAgentKit
```

#### Instance Methods
- `getBalance(): Promise<number>` - Gets wallet balance in lamports

### `Config` Interface

| Property | Type | Description |
|----------|------|-------------|
| `openaiApiKey` | `string?` | API key for OpenAI services |
| `jupiterReferralAccount` | `string?` | Jupiter aggregator referral account |
| `jupiterFeeBps` | `number?` | Fee basis points (0-10000) |
| `flashPrivilege` | `string?` | Flash loan privilege key |
| `flexlendApiKey` | `string?` | FlexLend service API key |
| `heliusApiKey` | `string?` | Helius RPC API key |
| `cookieApiKey` | `string?` | Cookie program access key |
| `birdeyeApiKey` | `string?` | Birdeye analytics API key |

## Requirements

- Node.js v16+
- TypeScript v4.9+ (recommended)
- @solana/web3.js ^1.100.0

## Development

```bash
# Install dependencies
npm install

# Build project
npm run build

# Run tests
npm test
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss proposed changes.

## License

Apache 2.0