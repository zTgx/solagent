# @solagent/wallet

[![npm version](https://img.shields.io/npm/v/@solagent/wallet.svg)](https://www.npmjs.com/package/@solagent/wallet)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A secure, type-safe Solana wallet management library for the SolAgent ecosystem. Easily generate, import, and manage Solana keypairs with robust error handling.

## Features

- 🔐 Generate new Solana wallets with one line of code
- 📁 Load wallets from environment variables, files, or raw private keys
- 🛡️ Type-safe implementation with comprehensive error handling
- 🔄 Convert between base58 and native key formats
- 📦 Lightweight with zero unnecessary dependencies

## Installation

```bash
npm install @solagent/wallet
# or
yarn add @solagent/wallet
```

## Usage

### Basic Wallet Operations

```typescript
import { Wallet } from '@solagent/wallet';

// Create a new wallet
const wallet = new Wallet();
console.log('Public key:', wallet.pubkey);
console.log('Private key:', wallet.toBase58());

// Import from private key
const importedWallet = Wallet.fromBase58(wallet.toBase58());
console.log('Imported public key:', importedWallet.pubkey);
```

### Environment Variables

```typescript
// .env file:
// PRIVATE_KEY=your_base58_encoded_private_key_here

const envWallet = Wallet.fromEnv('PRIVATE_KEY');
```

### File Operations

```typescript
// Save wallet to file
const wallet = new Wallet();
wallet.saveToFile('my-wallet.key');

// Load from file later
const loadedWallet = Wallet.fromFile('my-wallet.key');
console.log('Loaded public key:', loadedWallet.pubkey);
```

### Error Handling

```typescript
try {
  const wallet = Wallet.fromBase58('invalid_key');
} catch (error) {
  if (error instanceof WalletError) {
    console.error('Wallet error:', error.message);
  }
}
```

## API Reference

### `Wallet`
- `new Wallet()` - Creates a new random wallet
- `Wallet.fromBase58(privateKey: string)` - Creates wallet from base58 private key
- `Wallet.fromEnv(variableName: string)` - Creates wallet from environment variable
- `Wallet.fromFile(filePath: string)` - Creates wallet from key file

### Instance Properties
- `keypair: Keypair` - The underlying Solana keypair
- `pubkey: string` - Base58 encoded public key

### Instance Methods
- `toBase58(): string` - Returns private key as base58 string
- `saveToFile(filePath: string): void` - Saves private key to file

## Security Considerations

- Always store private keys securely
- Never commit key files or .env files to version control
- Consider using hardware wallets for production deployments

## Contributing

Pull requests are welcome! Please ensure:
- All tests pass (`npm test`)
- Code is properly typed
- New features include tests

## License

Apache 2.0