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

use dotenv::dotenv;
use solana_sdk::{bs58, pubkey::Pubkey, signature::Keypair, signer::Signer};
use std::env;

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
    pub fn from_env(variable_name: &str) -> Result<Self, String> {
        dotenv().ok(); // Load environment variables from .env file (if present)

        let private_key =
            env::var(variable_name).map_err(|_| format!("Environment variable '{}' not found", variable_name))?;

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
    pub fn from_base58(private_key: &str) -> Result<Self, String> {
        let secret_key = bs58::decode(private_key).into_vec().map_err(|_| "Invalid base58 private key".to_string())?;

        let keypair = Keypair::from_bytes(&secret_key).map_err(|_| "Invalid private key bytes".to_string())?;

        let pubkey = keypair.pubkey();
        Ok(Self { keypair, pubkey })
    }

    /// Returns the base58 encoded private key of the wallet.
    pub fn to_base58(&self) -> String {
        self.keypair.to_base58_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_wallet_from_env_valid() {
        // Set an environment variable for the test
        let keypair = Keypair::new();
        let private_key = keypair.to_base58_string();
        env::set_var("TEST_PRIVATE_KEY", &private_key);

        let wallet = Wallet::from_env("TEST_PRIVATE_KEY").unwrap();
        assert_eq!(wallet.pubkey, keypair.pubkey());

        // Clean up the environment variable after the test
        env::remove_var("TEST_PRIVATE_KEY");
    }

    #[test]
    fn test_wallet_from_env_not_found() {
        let result = Wallet::from_env("NON_EXISTENT_VARIABLE");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Environment variable 'NON_EXISTENT_VARIABLE' not found");
    }

    #[test]
    fn test_wallet_creation() {
        let wallet = Wallet::new();
        assert_ne!(wallet.pubkey, Pubkey::default());
    }

    #[test]
    fn test_wallet_from_base58_valid() {
        let original_keypair = Keypair::new();
        let private_key = original_keypair.to_base58_string();
        let wallet = Wallet::from_base58(&private_key).unwrap();
        assert_eq!(wallet.pubkey, original_keypair.pubkey());
    }

    #[test]
    fn test_wallet_from_base58_invalid_base58() {
        let result = Wallet::from_base58("invalid_key");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid base58 private key");
    }

    #[test]
    fn test_wallet_from_base58_invalid_bytes() {
        // Create a base58 string that's the wrong length to be a key
        let invalid_bytes = bs58::encode([0u8; 10]).into_string(); //Incorrect number of bytes
        let result = Wallet::from_base58(&invalid_bytes);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid private key bytes");
    }

    #[test]
    fn test_wallet_to_base58() {
        let wallet = Wallet::new();
        let base58_key = wallet.to_base58();
        assert!(!base58_key.is_empty());

        let wallet2 = Wallet::from_base58(&base58_key).unwrap();
        assert_eq!(wallet.pubkey, wallet2.pubkey);
    }
}
