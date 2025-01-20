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

use crate::{
    primitives::{
        close_empty_token_accounts::{CloseEmptyTokenAccountsData, Parsed},
        constants::USDC,
    },
    SolanaAgentKit,
};
use solana_client::rpc_request::TokenAccountsFilter;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, transaction::Transaction};
use spl_token::instruction::close_account;

/// Close Empty SPL Token accounts of the agent.
///
/// # Parameters
///
/// - `agent`: An instance of `SolanaAgentKit`.
///
/// # Returns
///
/// Transaction signature and total number of accounts closed or an error if the account doesn't exist.
pub async fn close_empty_token_accounts(
    agent: &SolanaAgentKit,
) -> Result<CloseEmptyTokenAccountsData, Box<dyn std::error::Error>> {
    let max_instructions = 40_u32;
    let mut transaction: Vec<Instruction> = vec![];
    let mut closed_size = 0;
    let token_programs = vec![spl_token::ID, spl_token_2022::ID];

    for token_program in token_programs {
        let accounts = agent
            .connection
            .get_token_accounts_by_owner(
                &agent.wallet.address,
                TokenAccountsFilter::ProgramId(token_program.to_owned()),
            )
            .expect("get_token_accounts_by_owner");

        closed_size += accounts.len();

        for account in accounts {
            if transaction.len() >= max_instructions as usize {
                break;
            }

            if let solana_account_decoder::UiAccountData::Json(d) = &account.account.data {
                if let Ok(parsed) = serde_json::from_value::<Parsed>(d.parsed.clone()) {
                    if parsed.info.token_amount.amount.parse::<u32>().unwrap_or_default() == 0_u32
                        && parsed.info.mint != USDC
                    {
                        let account_pubkey = Pubkey::from_str_const(&account.pubkey);
                        if let Ok(instruct) = close_account(
                            &token_program,
                            &account_pubkey,
                            &agent.wallet.address,
                            &agent.wallet.address,
                            &[&agent.wallet.address],
                        ) {
                            transaction.push(instruct);
                        }
                    }
                }
            }
        }
    }

    if transaction.is_empty() {
        return Ok(CloseEmptyTokenAccountsData::default());
    }

    // Create and send transaction
    let recent_blockhash = agent.connection.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &transaction,
        Some(&agent.wallet.address),
        &[&agent.wallet.wallet],
        recent_blockhash,
    );

    let signature = agent.connection.send_and_confirm_transaction(&transaction)?;
    let data = CloseEmptyTokenAccountsData::new(signature.to_string(), closed_size);
    Ok(data)
}
