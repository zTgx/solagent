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

use solana_sdk::{bs58, pubkey::Pubkey, signature::Keypair, signer::Signer};

pub struct Wallet {
    pub wallet: Keypair,
    pub address: Pubkey,
}

impl Wallet {
    pub fn load(private_key: &str) -> Wallet {
        let secret_key = bs58::decode(private_key).into_vec().expect("private key is not valid base58 format!");
        let wallet = Keypair::from_bytes(&secret_key).expect("Invalid private key!");
        let address = wallet.pubkey();

        Wallet { wallet, address }
    }
}
