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
    primitives::token::{DeployedData, NFTMetadata},
    SolanaAgentKit,
};
use mpl_token_metadata::{
    instructions::{CreateMasterEditionV3, CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs},
    types::{Collection, DataV2},
};
use solana_client::client_error::ClientError;
use solana_sdk::{
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    sysvar,
    transaction::Transaction,
};

/// Mints a new NFT
///
/// # Arguments
/// - `agent`: An instance of `SolanaAgentKit`.
/// - `collection`: The public key of the collection to which the NFT belongs. This is used to associate the NFT with a specific collection if applicable.
/// - `metadata`: A struct containing the NFT's metadata:
///     * `name`: The name of the NFT as a string.
///     * `uri`: A URI pointing to the NFT's assets (e.g., image, description) as a string.
///     * `seller_fee_basis_points`: An optional seller fee basis points as a number. This represents a percentage of the sale price (e.g., 500 means 5%).
///     * `creators`: An optional array of creator information. Each element contains the creator's address (as a string, to be converted to a `Pubkey` in practice) and their share (as a number, representing their contribution percentage).
///
/// # Returns
/// The transaction signature.
pub async fn mint_nft_to_collection(
    agent: &SolanaAgentKit,
    collection: Pubkey,
    metadata: NFTMetadata,
) -> Result<DeployedData, ClientError> {
    // Create a new keypair for the mint
    let mint_keypair = Keypair::new();
    let mint_pubkey = mint_keypair.pubkey();

    // Create token mint account
    let min_rent = agent.connection.get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)?;

    // Find the metadata account
    let metadata_seeds = &["metadata".as_bytes(), mpl_token_metadata::ID.as_ref(), mint_pubkey.as_ref()];
    let (metadata_account, _) = Pubkey::find_program_address(metadata_seeds, &mpl_token_metadata::ID);

    // Create the mint account
    let create_mint_account_ix = solana_sdk::system_instruction::create_account(
        &agent.wallet.address,
        &mint_pubkey,
        min_rent,
        82,
        &spl_token::id(),
    );

    // Initialize the mint
    let init_mint_ix = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &mint_pubkey,
        &agent.wallet.address,
        Some(&agent.wallet.address),
        0,
    )
    .unwrap();

    // Create Associated Token Account
    let associated_token_account =
        spl_associated_token_account::get_associated_token_address(&agent.wallet.address, &mint_pubkey);
    let create_assoc_account_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &agent.wallet.address,
        &agent.wallet.address,
        &mint_pubkey,
        &spl_token::id(),
    );

    // Mint one token
    let mint_to_ix = spl_token::instruction::mint_to(
        &spl_token::id(),
        &mint_pubkey,
        &associated_token_account,
        &agent.wallet.address,
        &[&agent.wallet.address],
        1,
    )
    .unwrap();

    // Create metadata account
    let create_metadata_ix = CreateMetadataAccountV3 {
        metadata: metadata_account,
        mint: mint_pubkey,
        mint_authority: agent.wallet.address,
        payer: agent.wallet.address,
        update_authority: (agent.wallet.address, false),
        system_program: solana_program::system_program::id(),
        rent: Some(sysvar::rent::id()),
    }
    .instruction(CreateMetadataAccountV3InstructionArgs {
        data: DataV2 {
            name: metadata.name.clone(),
            symbol: "SOLAGENT".to_string(),
            uri: metadata.uri.clone(),
            seller_fee_basis_points: metadata.basis_points.unwrap_or(0),
            creators: metadata.creators,
            collection: Some(Collection { verified: false, key: collection }),
            uses: None,
        },
        is_mutable: true,
        collection_details: None,
    });

    // Create master edition account
    let master_edition_seeds =
        &["metadata".as_bytes(), mpl_token_metadata::ID.as_ref(), mint_pubkey.as_ref(), "edition".as_bytes()];
    let (master_edition_account, _) = Pubkey::find_program_address(master_edition_seeds, &mpl_token_metadata::ID);

    let create_master_edition_ix = CreateMasterEditionV3 {
        edition: master_edition_account,
        mint: mint_pubkey,
        update_authority: agent.wallet.address,
        mint_authority: agent.wallet.address,
        payer: agent.wallet.address,
        metadata: metadata_account,
        token_program: spl_token::id(),
        system_program: solana_program::system_program::id(),
        rent: Some(sysvar::rent::id()),
    }
    .instruction(mpl_token_metadata::instructions::CreateMasterEditionV3InstructionArgs { max_supply: Some(1) });

    // Verify the collection
    use mpl_token_metadata::instructions::VerifyCollection;

    let collection_metadata_seeds = &["metadata".as_bytes(), mpl_token_metadata::ID.as_ref(), collection.as_ref()];
    let (collection_metadata_account, _) =
        Pubkey::find_program_address(collection_metadata_seeds, &mpl_token_metadata::ID);

    let collection_master_edition_seeds =
        &["metadata".as_bytes(), mpl_token_metadata::ID.as_ref(), collection.as_ref(), "edition".as_bytes()];
    let (collection_master_edition_account, _) =
        Pubkey::find_program_address(collection_master_edition_seeds, &mpl_token_metadata::ID);

    let verify_collection_ix = VerifyCollection {
        metadata: metadata_account,
        collection_authority: agent.wallet.address,
        payer: agent.wallet.address,
        collection_mint: collection,
        collection: collection_metadata_account,
        collection_master_edition_account,
        collection_authority_record: None,
    }
    .instruction();

    // Create the transaction
    let recent_blockhash = agent.connection.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[
            create_mint_account_ix,
            init_mint_ix,
            create_assoc_account_ix,
            mint_to_ix,
            create_metadata_ix,
            create_master_edition_ix,
            verify_collection_ix,
        ],
        Some(&agent.wallet.address),
        &[&agent.wallet.wallet, &mint_keypair],
        recent_blockhash,
    );

    // Send and confirm the transaction
    let signature = agent.connection.send_and_confirm_transaction(&transaction)?;
    Ok(DeployedData { mint: mint_pubkey.to_string(), signature: signature.to_string() })
}
