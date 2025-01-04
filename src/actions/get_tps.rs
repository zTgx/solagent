use crate::agent::SolAgent;
use solana_client::client_error::ClientError;

/// Gets the transactions per second (TPS) from the Solana network.
///
/// # Parameters
///
/// - `agent`: An instance of `RpcClient` that connects to the Solana cluster.
///
/// # Returns
///
/// A `Result` containing the TPS as a `f64`, or an error if fetching performance samples fails.
pub async fn get_tps(agent: &SolAgent) -> Result<f64, ClientError> {
    // Fetch recent performance samples
    let limit = 1;
    let perf_samples = agent
        .connection
        .get_recent_performance_samples(Some(limit))?;

    // Check if there are any samples available
    if !perf_samples.is_empty() {
        // Calculate TPS
        let num_transactions = perf_samples[0].num_transactions;
        let sample_period_secs = perf_samples[0].sample_period_secs;

        let tps = num_transactions as f64 / sample_period_secs as f64;

        return Ok(tps);
    }

    Ok(0.0)
}
