use solana_client::client_error::ClientError;
use solana_sdk::{
    program_pack::Pack, pubkey::Pubkey, system_instruction, transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::transfer as transfer_instruct;
use spl_token::state::Mint;

use crate::SolAgent;

/// Transfer SOL or SPL tokens to a recipient
///
/// `agent` - SolAgent instance
/// `to` - Recipient's public key
/// `amount` - Amount to transfer
/// `mint` - Optional mint address for SPL tokens
///
/// Returns the transaction signature.
pub async fn transfer(
    agent: &SolAgent,
    to: &str,
    amount: u64,
    mint: Option<String>,
) -> Result<String, ClientError> {
    match mint {
        Some(mint) => {
            // Transfer SPL Token
            let mint = Pubkey::from_str_const(&mint);
            let to = Pubkey::from_str_const(&to);

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

            let signature = agent
                .connection
                .send_and_confirm_transaction(&transaction)
                .unwrap();
            return Ok(signature.to_string());
        }
        None => {
            let transfer_instruction = system_instruction::transfer(
                &agent.wallet.address,
                &Pubkey::from_str_const(to),
                amount,
            );
            let transaction = Transaction::new_signed_with_payer(
                &[transfer_instruction],
                Some(&agent.wallet.address),
                &[&agent.wallet.wallet],
                agent.connection.get_latest_blockhash().unwrap(),
            );

            let signature = agent
                .connection
                .send_and_confirm_transaction(&transaction)
                .unwrap();
            return Ok(signature.to_string());
        }
    }
}
