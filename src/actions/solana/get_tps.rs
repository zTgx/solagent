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

use crate::SolanaAgentKit;
use solana_client::client_error::ClientError;

/// Gets the transactions per second (TPS) from the Solana network.
///
/// # Parameters
///
/// - `agent`: An instance of `SolanaAgentKit` that connects to the Solana cluster.
///
/// # Returns
///
/// A `Result` containing the TPS as a `f64`, or an error if fetching performance samples fails.
pub async fn get_tps(agent: &SolanaAgentKit) -> Result<f64, ClientError> {
    // Fetch recent performance samples
    let limit = 1;
    let perf_samples = agent.connection.get_recent_performance_samples(Some(limit))?;

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
