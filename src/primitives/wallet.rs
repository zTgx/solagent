use solana_sdk::bs58;
use solana_sdk::signature::Signer;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

pub struct Wallet {
    pub wallet: Keypair,
    pub address: Pubkey,
}

impl Wallet {
    pub fn load(private_key: &str) -> Wallet {
        let secret_key = bs58::decode(private_key)
            .into_vec()
            .expect("private key is not valid base58 format!");
        let wallet = Keypair::from_bytes(&secret_key).expect("Invalid private key!");
        let address = wallet.pubkey();

        Wallet { wallet, address }
    }
}
