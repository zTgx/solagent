use crate::agent::SolAgent;
use solana_client::client_error::ClientError;
use solana_sdk::native_token::LAMPORTS_PER_SOL;

/// Requests SOL from the Solana faucet (devnet/testnet only).
///
/// # Parameters
///
/// - `agent`: An instance of `SolanaAgentKit`.
///
/// # Returns
///
/// A transaction signature as a `String`.
///
/// # Errors
///
/// Returns an error if the request fails or times out.
pub async fn request_faucet_funds(agent: &SolAgent) -> Result<String, ClientError> {
    // Request airdrop of 5 SOL (5 * LAMPORTS_PER_SOL)
    let tx = agent
        .connection
        .request_airdrop(&agent.wallet.address, 5 * LAMPORTS_PER_SOL)?;

    // Confirm the transaction
    agent.connection.confirm_transaction(&tx)?;

    Ok(tx.to_string())
}
