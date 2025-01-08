use crate::agent::SolAgent;
use crate::primitives::token::NftMetadata;
use mpl_token_metadata::instructions::{
    CreateMasterEditionV3, CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs,
};
use mpl_token_metadata::types::DataV2;
use solana_client::client_error::ClientError;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use solana_sdk::{system_instruction, sysvar};
use spl_associated_token_account::instruction::create_associated_token_account;

/// Deploys a new NFT collection.
///
/// # Parameters
///
/// - `agent`: An instance of `SolAgent`.
/// - `options`: Collection options including name, URI, royalties, and creators.
///
/// # Returns
///
/// An object containing the collection address and metadata.
pub async fn deploy_collection(
    agent: &SolAgent,
    options: &NftMetadata,
) -> Result<(String, String), ClientError> {
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
        &agent.wallet.address,
        &collection_mint_pubkey,
    );

    // let create_mint_account_ix = system_instruction::create_account(
    //     &agent.wallet.address,
    //     &collection_mint.pubkey(),
    //     min_rent,
    //     spl_token::state::Mint::LEN as u64,
    //     &spl_token::id(),
    // );
    // // Initialize mint
    // let init_mint_ix = spl_token::instruction::initialize_mint(
    //     &spl_token::id(),
    //     &collection_mint_pubkey,
    //     &agent.wallet.address,
    //     Some(&agent.wallet.address),
    //     0,
    // )
    // .unwrap();

    // Create associated token account
    let create_assoc_account_ix = create_associated_token_account(
        &agent.wallet.address,
        &agent.wallet.address,
        &collection_mint_pubkey,
        &spl_token::id(),
    );

    // Mint one token to the associated token account
    let mint_to_ix = spl_token::instruction::mint_to(
        &spl_token::id(),
        &collection_mint.pubkey(),
        &associated_token_account,
        &agent.wallet.address,
        &[&agent.wallet.address],
        1,
    )
    .unwrap();

    // Create metadata
    let create_metadata_ix = CreateMetadataAccountV3 {
        metadata: metadata_account,
        mint: collection_mint.pubkey(),
        mint_authority: agent.wallet.address,
        payer: agent.wallet.address,
        update_authority: (agent.wallet.address, false),
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
        update_authority: agent.wallet.address,
        mint_authority: agent.wallet.address,
        payer: agent.wallet.address,
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
        &agent.wallet.address,
        &collection_mint.pubkey(),
        min_rent,
        82,
        &spl_token::id(),
    );

    // Initialize mint
    let init_mint_ix = spl_token::instruction::initialize_mint(
        &spl_token::id(),
        &collection_mint.pubkey(),
        &&agent.wallet.address,
        Some(&&agent.wallet.address),
        0,
    )
    .unwrap();

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
        Some(&&agent.wallet.address),
        &[&agent.wallet.wallet, &collection_mint],
        recent_blockhash,
    );

    let signature = agent
        .connection
        .send_and_confirm_transaction(&transaction)?;

    Ok((collection_mint_pubkey.to_string(), signature.to_string()))
}
