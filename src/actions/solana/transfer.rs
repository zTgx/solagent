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

use crate::SolanaAgentKit;
use solana_client::client_error::ClientError;
use solana_sdk::{program_pack::Pack, pubkey::Pubkey, system_instruction, transaction::Transaction};
use spl_associated_token_account::get_associated_token_address;
use spl_token::{instruction::transfer as transfer_instruct, state::Mint};

/// Transfer SOL or SPL tokens to a recipient
///
/// `agent` - SolanaAgentKit instance
/// `to` - Recipient's public key
/// `amount` - Amount to transfer
/// `mint` - Optional mint address for SPL tokens
///
/// Returns the transaction signature.
pub async fn transfer(
    agent: &SolanaAgentKit,
    to: &str,
    amount: u64,
    mint: Option<String>,
) -> Result<String, ClientError> {
    match mint {
        Some(mint) => {
            // Transfer SPL Token
            let mint = Pubkey::from_str_const(&mint);
            let to = Pubkey::from_str_const(to);

            let from_ata = get_associated_token_address(&mint, &agent.wallet.address);
            let to_ata = get_associated_token_address(&mint, &to);

            let account_info = &agent.connection.get_account(&mint).unwrap();
            let mint_info = Mint::unpack_from_slice(&account_info.data).unwrap();

            let adjusted_amount = amount * 10u64.pow(mint_info.decimals as u32);

            let transfer_instruction = transfer_instruct(
                &spl_token::id(),
                &from_ata,
                &to_ata,
                &from_ata,
                &[&agent.wallet.address],
                adjusted_amount,
            )
            .unwrap();

            let transaction = Transaction::new_signed_with_payer(
                &[transfer_instruction],
                Some(&agent.wallet.address),
                &[&agent.wallet.wallet],
                agent.connection.get_latest_blockhash().unwrap(),
            );

            let signature = agent.connection.send_and_confirm_transaction(&transaction).unwrap();
            Ok(signature.to_string())
        }
        None => {
            let transfer_instruction =
                system_instruction::transfer(&agent.wallet.address, &Pubkey::from_str_const(to), amount);
            let transaction = Transaction::new_signed_with_payer(
                &[transfer_instruction],
                Some(&agent.wallet.address),
                &[&agent.wallet.wallet],
                agent.connection.get_latest_blockhash().unwrap(),
            );

            let signature = agent.connection.send_and_confirm_transaction(&transaction).unwrap();
            Ok(signature.to_string())
        }
    }
}
