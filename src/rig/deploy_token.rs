use crate::agent::SolAgent;
use solana_client::client_error::ClientError;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_client::rpc_client::RpcClient;
use solana_program::system_instruction;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction as spl_token_instruction;

pub struct SolanaAgentKit {
    pub rpc_client: RpcClient,
    pub wallet: Keypair,
}

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
        .get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN).await?;

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
    ).expect("initialize_mint");

    // Create metadata account
    let metadata_account = mpl_token_metadata::find_metadata_account(&mint_pubkey).0;
    let create_metadata_ix = mpl_token_metadata::instructions::CreateMetadataAccountV3Cpi{
        mpl_token_metadata::ID,
        metadata_account,
        mint_pubkey,
        agent.wallet.address,
        agent.wallet.address,
        agent.wallet.address,
        name,
        symbol,
        uri,
        None,
        0,
        true,
        true,
        None,
        None,
        None,
    };

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
        ).expect("mint_to");

        instructions.push(create_associated_token_account_ix);
        instructions.push(mint_to_ix);
    }

    let recent_blockhash = agent.connection.get_latest_blockhash().await?;
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
        ).await?;

    Ok(mint_pubkey)
}