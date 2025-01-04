use crate::agent::SolAgent;
use mpl_token_metadata::accounts::Metadata;
use mpl_token_metadata::instructions::{CreateV1, CreateV1InstructionArgs};
use mpl_token_metadata::types::{PrintSupply, TokenStandard};
use solana_client::client_error::ClientError;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::{commitment_config::CommitmentConfig, transaction::Transaction};
use solana_sdk::{system_instruction, system_program};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction as spl_token_instruction;

/// Deploys a new SPL token.
///
/// # Parameters
///
/// - `agent`: An instance of `SolanaAgent`.
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
    agent: &SolAgent,
    name: String,
    uri: String,
    symbol: String,
    decimals: u8,
    initial_supply: Option<u64>,
) -> Result<Pubkey, ClientError> {
    let mint = Keypair::new();
    let mint_pubkey = mint.pubkey();

    // Create token mint account
    let min_rent = agent
        .connection
        .get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)?;

    let create_mint_account_ix = system_instruction::create_account(
        &agent.wallet.address,
        &mint_pubkey,
        min_rent,
        spl_token::state::Mint::LEN as u64,
        &spl_token::id(),
    );

    let initialize_mint_ix = spl_token_instruction::initialize_mint(
        &spl_token::id(),
        &mint_pubkey,
        &agent.wallet.address,
        Some(&agent.wallet.address),
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
        authority: agent.wallet.address,
        payer: agent.wallet.address,
        update_authority: (agent.wallet.address, true),
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
            get_associated_token_address(&agent.wallet.address, &mint_pubkey);

        let create_associated_token_account_ix =
            spl_associated_token_account::instruction::create_associated_token_account(
                &agent.wallet.address,
                &agent.wallet.address,
                &mint_pubkey,
                &spl_token::id(),
            );

        let mint_to_ix = spl_token_instruction::mint_to(
            &spl_token::id(),
            &mint_pubkey,
            &associated_token_account,
            &agent.wallet.address,
            &[&agent.wallet.address],
            supply,
        )
        .expect("mint_to");

        instructions.push(create_associated_token_account_ix);
        instructions.push(mint_to_ix);
    }

    let recent_blockhash = agent.connection.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&agent.wallet.address),
        &[&agent.wallet.wallet, &mint],
        recent_blockhash,
    );

    agent
        .connection
        .send_and_confirm_transaction_with_spinner_and_config(
            &transaction,
            CommitmentConfig::finalized(),
            solana_client::rpc_config::RpcSendTransactionConfig {
                skip_preflight: true,
                ..Default::default()
            },
        )?;

    Ok(mint_pubkey)
}
