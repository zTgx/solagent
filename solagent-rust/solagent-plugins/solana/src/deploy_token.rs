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

use crate::DeployedData;
use mpl_token_metadata::{
    accounts::Metadata,
    instructions::{CreateV1, CreateV1InstructionArgs},
    types::{PrintSupply, TokenStandard},
};
use solagent_core::{
    solana_client::{client_error::ClientError, rpc_config::RpcSendTransactionConfig},
    solana_program,
    solana_sdk::{
        program_pack::Pack,
        signature::{Keypair, Signer},
        system_instruction, system_program,
        {commitment_config::CommitmentConfig, transaction::Transaction},
    },
    SolanaAgentKit,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction as spl_token_instruction;

/// Deploys a new SPL token.
///
/// # Parameters
///
/// - `agent`: An instance of `SolanaAgentKit`.
/// - `name`: Name of the token.
/// - `uri`: URI for the token metadata.
/// - `symbol`: Symbol of the token.
/// - `decimals`: Number of decimals for the token (default: 9).
/// - `initial_supply`: Initial supply to mint (optional).
///
/// # Returns
///
/// An object containing the token mint address.
pub async fn deploy_token(
    agent: &SolanaAgentKit,
    name: String,
    uri: String,
    symbol: String,
    decimals: u8,
    initial_supply: Option<u64>,
) -> Result<DeployedData, ClientError> {
    let mint = Keypair::new();
    let mint_pubkey = mint.pubkey();

    // Create token mint account
    let min_rent = agent
        .connection
        .get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)?;

    let create_mint_account_ix = system_instruction::create_account(
        &agent.wallet.pubkey,
        &mint_pubkey,
        min_rent,
        spl_token::state::Mint::LEN as u64,
        &spl_token::id(),
    );

    let initialize_mint_ix = spl_token_instruction::initialize_mint(
        &spl_token::id(),
        &mint_pubkey,
        &agent.wallet.pubkey,
        Some(&agent.wallet.pubkey),
        decimals,
    )
    .expect("initialize_mint");

    // Create metadata account
    let (metadata, _x) = Metadata::find_pda(&mint_pubkey);
    // instruction args
    let args = CreateV1InstructionArgs {
        name,
        symbol,
        uri,
        seller_fee_basis_points: 500,
        primary_sale_happened: false,
        is_mutable: true,
        token_standard: TokenStandard::Fungible,
        collection: None,
        uses: None,
        collection_details: None,
        creators: None,
        rule_set: None,
        decimals: Some(18),
        print_supply: Some(PrintSupply::Zero),
    };

    // instruction accounts
    let create_ix = CreateV1 {
        metadata,
        master_edition: None,
        mint: (mint_pubkey, true),
        authority: agent.wallet.pubkey,
        payer: agent.wallet.pubkey,
        update_authority: (agent.wallet.pubkey, true),
        system_program: system_program::ID,
        sysvar_instructions: solana_program::sysvar::instructions::ID,
        spl_token_program: Some(spl_token::ID),
    };
    let create_metadata_ix = create_ix.instruction(args);

    let mut instructions = vec![
        create_mint_account_ix,
        initialize_mint_ix,
        create_metadata_ix,
    ];

    if let Some(supply) = initial_supply {
        let associated_token_account =
            get_associated_token_address(&agent.wallet.pubkey, &mint_pubkey);

        let create_associated_token_account_ix =
            spl_associated_token_account::instruction::create_associated_token_account(
                &agent.wallet.pubkey,
                &agent.wallet.pubkey,
                &mint_pubkey,
                &spl_token::id(),
            );

        let mint_to_ix = spl_token_instruction::mint_to(
            &spl_token::id(),
            &mint_pubkey,
            &associated_token_account,
            &agent.wallet.pubkey,
            &[&agent.wallet.pubkey],
            supply,
        )
        .expect("mint_to");

        instructions.push(create_associated_token_account_ix);
        instructions.push(mint_to_ix);
    }

    let recent_blockhash = agent.connection.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&agent.wallet.pubkey),
        &[&agent.wallet.keypair, &mint],
        recent_blockhash,
    );

    let signature = agent
        .connection
        .send_and_confirm_transaction_with_spinner_and_config(
            &transaction,
            CommitmentConfig::finalized(),
            RpcSendTransactionConfig {
                skip_preflight: true,
                ..Default::default()
            },
        )?;

    Ok(DeployedData::new(
        mint_pubkey.to_string(),
        signature.to_string(),
    ))
}
