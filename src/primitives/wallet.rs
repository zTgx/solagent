use serde_json::Value;
use solana_sdk::signature::Signer;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::fs;

pub struct Wallet {
    pub wallet: Keypair,
    pub address: Pubkey,
}

impl Wallet {
    pub fn load(wallet_path: &str) -> Wallet {
        let wallet_path = shellexpand::tilde(wallet_path).to_string();
        let file_content = fs::read_to_string(wallet_path).expect("Unable to read wallet file");
        let secrets: Value = serde_json::from_str(&file_content).expect("Unable to parse JSON");
        let secret_key: Vec<u8> = secrets
            .as_array()
            .expect("Expected an array")
            .iter()
            .map(|v| v.as_u64().expect("Expected a number") as u8)
            .collect();

        let wallet = Keypair::from_bytes(&secret_key).expect("Invalid secret key length");
        let address = wallet.pubkey();

        Wallet { wallet, address }
    }
}
