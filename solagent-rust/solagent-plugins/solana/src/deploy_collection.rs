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

use crate::{DeployedData, NFTMetadata};
use mpl_token_metadata::{
    instructions::{
        CreateMasterEditionV3, CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs,
    },
    types::DataV2,
};
use solagent_core::{
    solana_client::client_error::ClientError,
    solana_program,
    solana_sdk::{
        program_pack::Pack,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        system_instruction, sysvar,
        transaction::Transaction,
    },
    SolanaAgentKit,
};
use spl_associated_token_account::instruction::create_associated_token_account;

/// Deploys a new NFT collection.
///
/// # Parameters
///
/// - `agent`: An instance of `SolanaAgentKit`.
/// - `options`: Collection options including name, URI, royalties, and creators.
///
/// # Returns
///
/// An object containing the collection address and metadata.
pub async fn deploy_collection(
    agent: &SolanaAgentKit,
    options: &NFTMetadata,
) -> Result<DeployedData, ClientError> {
    // Create a new mint for the collection
    let collection_mint = Keypair::new();
    let collection_mint_pubkey = collection_mint.pubkey();

    // Create token mint account
    let min_rent = agent
        .connection
        .get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)?;

    // Create metadata account
    let metadata_seeds = &[
        "metadata".as_bytes(),
        mpl_token_metadata::ID.as_ref(),
        collection_mint_pubkey.as_ref(),
    ];
    let (metadata_account, _) =
        Pubkey::find_program_address(metadata_seeds, &mpl_token_metadata::ID);

    // Create master edition account
    let master_edition_seeds = &[
        "metadata".as_bytes(),
        mpl_token_metadata::ID.as_ref(),
        collection_mint_pubkey.as_ref(),
        "edition".as_bytes(),
    ];
    let (master_edition_account, _) =
        Pubkey::find_program_address(master_edition_seeds, &mpl_token_metadata::ID);

    // Create associated token account for the mint
    let associated_token_account = spl_associated_token_account::get_associated_token_address(
        &agent.wallet.pubkey,
        &collection_mint_pubkey,
    );

    // let create_mint_account_ix = system_instruction::create_account(
    //     &agent.wallet.pubkey,
    //     &collection_mint.pubkey(),
    //     min_rent,
    //     spl_token::state::Mint::LEN as u64,
    //     &spl_token::id(),
    // );
    // // Initialize mint
    // let init_mint_ix = spl_token::instruction::initialize_mint(
    //     &spl_token::id(),
    //     &collection_mint_pubkey,
    //     &agent.wallet.pubkey,
    //     Some(&agent.wallet.pubkey),
    //     0,
    // )
    // .unwrap();

    // Create associated token account
    let create_assoc_account_ix = create_associated_token_account(
        &agent.wallet.pubkey,
        &agent.wallet.pubkey,
        &collection_mint_pubkey,
        &spl_token::id(),
    );

    // Mint one token to the associated token account
    let mint_to_ix = spl_token::instruction::mint_to(
        &spl_token::id(),
        &collection_mint.pubkey(),
        &associated_token_account,
        &agent.wallet.pubkey,
        &[&agent.wallet.pubkey],
        1,
    )
    .expect("mint_to");

    // Create metadata
    let create_metadata_ix = CreateMetadataAccountV3 {
        metadata: metadata_account,
        mint: collection_mint.pubkey(),
        mint_authority: agent.wallet.pubkey,
        payer: agent.wallet.pubkey,
        update_authority: (agent.wallet.pubkey, false),
        system_program: solana_program::system_program::id(),
        rent: Some(sysvar::rent::id()),
    }
    .instruction(CreateMetadataAccountV3InstructionArgs {
        data: DataV2 {
            name: options.name.clone(),
            symbol: "SOLAGENT".to_string(),
            uri: options.uri.clone(),
            seller_fee_basis_points: options.basis_points.unwrap_or(0),
            creators: options.creators.clone(),
            collection: None,
            uses: None,
        },
        is_mutable: true,
        collection_details: None,
    });

    // Create master edition
    let create_master_edition_ix = CreateMasterEditionV3 {
        edition: master_edition_account,
        mint: collection_mint.pubkey(),
        update_authority: agent.wallet.pubkey,
        mint_authority: agent.wallet.pubkey,
        payer: agent.wallet.pubkey,
        metadata: metadata_account,
        token_program: spl_token::id(),
        system_program: solana_program::system_program::id(),
        rent: Some(sysvar::rent::id()),
    }
    .instruction(
        mpl_token_metadata::instructions::CreateMasterEditionV3InstructionArgs {
            max_supply: Some(0),
        },
    ); // Max supply, 0 means unlimited

    // Create mint account
    let create_mint_account_ix = system_instruction::create_account(
        &agent.wallet.pubkey,
        &collection_mint.pubkey(),
        min_rent,
        82,
        &spl_token::id(),
    );

    // Initialize mint
    let init_mint_ix = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &collection_mint.pubkey(),
        &agent.wallet.pubkey,
        Some(&agent.wallet.pubkey),
        0,
    )
    .expect("initialize_mint");

    // Create and send transaction
    let recent_blockhash = agent.connection.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[
            create_mint_account_ix,
            init_mint_ix,
            create_assoc_account_ix,
            mint_to_ix,
            create_metadata_ix,
            create_master_edition_ix,
        ],
        Some(&agent.wallet.pubkey),
        &[&agent.wallet.keypair, &collection_mint],
        recent_blockhash,
    );

    let signature = agent
        .connection
        .send_and_confirm_transaction(&transaction)?;

    Ok(DeployedData::new(
        collection_mint_pubkey.to_string(),
        signature.to_string(),
    ))
}
