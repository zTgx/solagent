use crate::agent::SolAgent;

/// Get the agent's wallet address.
///
/// # Parameters
/// - `agent`: A `SolAgent` instance.
///
/// # Returns
/// A string representing the wallet address in base58 format.
pub fn get_wallet_address(agent: &SolAgent) -> String {
    agent.wallet.address.to_string()
}
