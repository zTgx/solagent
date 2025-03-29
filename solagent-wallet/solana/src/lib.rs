// Copyright 2025 zTgx
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::{Context, Result};
use dotenv::dotenv;
use solana_sdk::{bs58, pubkey::Pubkey, signature::Keypair, signer::Signer};
use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WalletError {
    #[error("Environment variable '{0}' not found")]
    EnvVarNotFound(String),
    #[error("Invalid base58 private key")]
    InvalidBase58Key,
    #[error("Invalid private key bytes")]
    InvalidPrivateKeyBytes,
    #[error("File operation failed: {0}")]
    FileError(String),
}

/// Represents a wallet containing a keypair and its corresponding public key.
#[derive(Debug)]
pub struct Wallet {
    /// The keypair associated with the wallet.  This contains the private key.
    pub keypair: Keypair,
    /// The public key associated with the wallet.
    pub pubkey: Pubkey,
}

impl Default for Wallet {
    /// Creates a new wallet with a randomly generated keypair.
    fn default() -> Self {
        Self::new()
    }
}

impl Wallet {
    /// Creates a new wallet with a randomly generated keypair.
    pub fn new() -> Self {
        let keypair = Keypair::new();
        let pubkey = keypair.pubkey();
        Self { keypair, pubkey }
    }

    /// Creates a wallet from a private key stored in an environment variable.
    ///
    /// This function reads the environment variable specified by `variable_name`,
    /// decodes the base58 encoded private key, and creates a `Wallet` instance.
    ///
    /// # Arguments
    ///
    /// * `variable_name` - The name of the environment variable containing the private key.
    ///
    /// # Returns
    ///
    /// * `Ok(Wallet)` - If the wallet was successfully created.
    /// * `Err(String)` - If the environment variable is not found or the private key is invalid.
    pub fn from_env(variable_name: &str) -> Result<Self> {
        dotenv().ok(); // Load environment variables from .env file (if present)

        let private_key =
            env::var(variable_name).with_context(|| format!("Environment variable '{}' not found", variable_name))?;

        Self::from_base58(&private_key)
    }

    /// Creates a wallet from a base58 encoded private key.
    ///
    /// # Arguments
    ///
    /// * `private_key` - The base58 encoded private key.
    ///
    /// # Returns
    ///
    /// * `Ok(Wallet)` - If the wallet was successfully created.
    /// * `Err(String)` - If the private key is invalid or not properly encoded.
    pub fn from_base58(private_key: &str) -> Result<Self> {
        let secret_key = bs58::decode(private_key).into_vec().map_err(|_| WalletError::InvalidBase58Key)?;

        let keypair = Keypair::from_bytes(&secret_key).map_err(|_| WalletError::InvalidPrivateKeyBytes)?;

        let pubkey = keypair.pubkey();
        Ok(Self { keypair, pubkey })
    }

    /// Returns the base58 encoded private key of the wallet.
    pub fn to_base58(&self) -> String {
        self.keypair.to_base58_string()
    }

    /// Saves the wallet's private key to a file.
    pub fn save_to_file(&self, file_path: &str) -> Result<()> {
        let private_key = self.to_base58();
        std::fs::write(file_path, private_key)
            .with_context(|| format!("Failed to save wallet to file: {}", file_path))?;
        Ok(())
    }

    /// Loads a wallet from a private key file.
    pub fn from_file(file_path: &str) -> Result<Self> {
        let private_key =
            std::fs::read_to_string(file_path).with_context(|| format!("Failed to read wallet file: {}", file_path))?;
        Self::from_base58(&private_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_wallet_from_env_valid() -> Result<()> {
        let keypair = Keypair::new();
        let private_key = keypair.to_base58_string();
        env::set_var("TEST_PRIVATE_KEY", &private_key);

        let wallet = Wallet::from_env("TEST_PRIVATE_KEY")?;
        assert_eq!(wallet.pubkey, keypair.pubkey());

        env::remove_var("TEST_PRIVATE_KEY");
        Ok(())
    }

    #[test]
    fn test_wallet_from_env_not_found() {
        let result = Wallet::from_env("NON_EXISTENT_VARIABLE");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Environment variable 'NON_EXISTENT_VARIABLE' not found");
    }

    #[test]
    fn test_wallet_creation() {
        let wallet = Wallet::new();
        assert_ne!(wallet.pubkey, Pubkey::default());
    }

    #[test]
    fn test_wallet_from_base58_valid() -> Result<()> {
        let original_keypair = Keypair::new();
        let private_key = original_keypair.to_base58_string();
        let wallet = Wallet::from_base58(&private_key)?;
        assert_eq!(wallet.pubkey, original_keypair.pubkey());
        Ok(())
    }

    #[test]
    fn test_wallet_from_base58_invalid_base58() {
        let result = Wallet::from_base58("invalid_key");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid base58 private key");
    }

    #[test]
    fn test_wallet_from_base58_invalid_bytes() {
        let invalid_bytes = bs58::encode([0u8; 10]).into_string();
        let result = Wallet::from_base58(&invalid_bytes);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid private key bytes");
    }

    #[test]
    fn test_wallet_to_base58() -> Result<()> {
        let wallet = Wallet::new();
        let base58_key = wallet.to_base58();
        assert!(!base58_key.is_empty());

        let wallet2 = Wallet::from_base58(&base58_key)?;
        assert_eq!(wallet.pubkey, wallet2.pubkey);
        Ok(())
    }
}
