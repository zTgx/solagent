use crate::agent::SolAgent;
use solana_client::{client_error::ClientError, rpc_request::TokenAccountsFilter};
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};

/// Gets the balance of SOL or an SPL token for the specified wallet address.
///
/// # Parameters
///
/// - `agent`: An instance of `SolAgent`, which contains the connection to the Solana cluster.
/// - `wallet_address`: The public key of the wallet to check balance for.
/// - `token_address`: An optional SPL token mint address. If not provided, returns SOL balance.
///
/// # Returns
///
/// A `Result` containing the balance as a number (in UI units) or an error if fetching fails.
pub async fn get_balance_other(
    agent: &SolAgent,
    wallet_address: Pubkey,
    token_address: Option<Pubkey>,
) -> Result<f64, ClientError> {
    if let Some(token_address) = token_address {
        // Get token accounts by owner for the specified token mint address
        let token_accounts = agent
            .connection
            .get_token_accounts_by_owner(&wallet_address, TokenAccountsFilter::Mint(token_address))
            .expect("get_token_accounts_by_owner");

        if token_accounts.is_empty() {
            println!(
                "No token accounts found for wallet {} and token {}",
                wallet_address, token_address
            );
            return Ok(0.0);
        }

        // Get the first token account's parsed account info
        let lamports = token_accounts[0].account.lamports;
        return Ok(lamports as f64 / LAMPORTS_PER_SOL as f64);
    } else {
        // Get SOL balance if no token address is provided
        let balance = agent.get_balance(Some(wallet_address.to_string())).await?;
        return Ok(balance);
    }
}
