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

use solagent_core::SolanaAgentKit;

/// Get the agent's wallet address.
///
/// # Parameters
/// - `agent`: A `SolanaAgentKit` instance.
///
/// # Returns
/// A string representing the wallet address in base58 format.
pub fn get_wallet_address(agent: &SolanaAgentKit) -> String {
    agent.wallet.pubkey.to_string()
}
