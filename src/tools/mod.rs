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

pub mod deploy_collection;
pub mod deploy_token;
pub mod fetch_price;
pub mod get_balance;
pub mod get_balance_other;
pub mod get_tps;
pub mod get_wallet_address;
pub mod pyth_fetch_price;
pub mod request_faucet_funds;
pub mod stake_with_jup;
pub mod transfer;

use super::{
    deploy_token::DeployToken, fetch_price::FetchPrice, get_balance::GetBalance, get_balance_other::GetBalanceOther,
    get_tps::GetTps, get_wallet_address::GetWalletAddress, pyth_fetch_price::FetchPricePyTh,
    request_faucet_funds::RequestFaucetFunds, transfer::Transfer,
};
use crate::SolAgent;
use rig::tool::ToolSet;
use std::sync::Arc;

/// An enumeration representing a set of tools that can be used with Solana.
///
/// This enum encapsulates different tool types, including:
/// - `GetBalance`: Tool to get the balance of a specified wallet.
/// - `GetBalanceOther`: Tool to get the balance of another wallet.
/// - `RequestFaucetFunds`: Tool to request funds from a faucet.
pub fn create_solana_tools(agent: Arc<SolAgent>) -> ToolSet {
    let builder = ToolSet::builder()
        .dynamic_tool(GetBalance::new(agent.clone()))
        .dynamic_tool(GetBalanceOther::new(agent.clone()))
        .dynamic_tool(RequestFaucetFunds::new(agent.clone()))
        .dynamic_tool(DeployToken::new(agent.clone()))
        .dynamic_tool(FetchPrice::new())
        .dynamic_tool(FetchPricePyTh::new())
        .dynamic_tool(GetTps::new(agent.clone()))
        .dynamic_tool(GetWalletAddress::new(agent.clone()))
        .dynamic_tool(Transfer::new(agent.clone()));

    builder.build()
}
