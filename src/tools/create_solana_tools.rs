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
use rig::tool::ToolSet;
use std::sync::Arc;

use super::{
    deploy_collection, deploy_token, fetch_price, get_balance_other, get_wallet_address, gibwork,
    jupiter::get_token_data_by_address,
    launch_token_pumpfun, mint_nft, pyth_fetch_price, rugcheck,
    solana::{close_empty_token_accounts, get_balance, get_tps, request_faucet_funds, transfer},
    solayer::stake_with_solayer,
    stake_with_jup, trade,
};

/// An function to build a set of tools that can be used with Solana.
///
/// - `DeployCollection`: Tool to deploy a new NFT collection on Solana blockchain.
/// - `DeployToken`: Tool to deploy a new token on Solana blockchain.
/// - `FetchPrice`: Tool to fetch the price of a given token in USDC.
/// - `GetBalance`: Tool to get the balance of a specified wallet.
/// - `GetBalanceOther`: Tool to get the balance of another wallet.
/// - `GetTps`: Tool to get the current TPS of the Solana network.
/// - `GetWalletAddress`: Tool to the wallet address of the agent.
/// - `LaunchPumpfunToken`: Tool to launch a token on Pump.fun.
/// - `MintNFT`: Tool to mint a new NFT in a collection on Solana blockchain.
/// - `FetchPricePyTh`: Tool to fetch the price of a given price feed from Pyth's Hermes service.
/// - `RequestFaucetFunds`: Tool to request funds from a faucet.
/// - `StakeWithJup`: Tool to stake your SOL (Solana), also called as SOL staking or liquid staking.
/// - `Trade`: Tool to swap tokens to another token ( It uses Jupiter Exchange ).
/// - `Transfer`: Tool to transfer tokens or SOL to another address ( also called as wallet address ).
/// - `FetchTokenReportSummary`: Tool to fetch a summary report for a specific token from RugCheck.
/// - `FetchTokenReportDetailed`: Tool to fetch a detailed report for a specific token from RugCheck.
/// - `CreateGibworkTask`: Tool to create a task on Gibwork.
/// - `CloseEmptyTokenAccounts`: Tool to close empty SPL Token accounts associated with your wallet to reclaim rent.
/// - `StakeWithSolayer`: Tool to stake native SOL with Solayer's restaking protocol to receive Solayer SOL (sSOL).
/// - `GetTokenData`: Tool to get the token data for a given token mint address.
pub fn create_solana_tools(agent: Arc<SolanaAgentKit>) -> ToolSet {
    let builder = ToolSet::builder()
        .dynamic_tool(deploy_collection::DeployCollection::new(agent.clone()))
        .dynamic_tool(deploy_token::DeployToken::new(agent.clone()))
        .dynamic_tool(fetch_price::FetchPrice::new())
        .dynamic_tool(get_balance::GetBalance::new(agent.clone()))
        .dynamic_tool(get_balance_other::GetBalanceOther::new(agent.clone()))
        .dynamic_tool(get_tps::GetTps::new(agent.clone()))
        .dynamic_tool(get_wallet_address::GetWalletAddress::new(agent.clone()))
        .dynamic_tool(launch_token_pumpfun::LaunchPumpfunToken::new(agent.clone()))
        .dynamic_tool(mint_nft::MintNFT::new(agent.clone()))
        .dynamic_tool(pyth_fetch_price::FetchPricePyTh::new())
        .dynamic_tool(request_faucet_funds::RequestFaucetFunds::new(agent.clone()))
        .dynamic_tool(stake_with_jup::StakeWithJup::new(agent.clone()))
        .dynamic_tool(trade::Trade::new(agent.clone()))
        .dynamic_tool(transfer::Transfer::new(agent.clone()))
        .dynamic_tool(rugcheck::token_report_summary::FetchTokenReportSummary::new())
        .dynamic_tool(rugcheck::token_report_detailed::FetchTokenReportDetailed::new())
        .dynamic_tool(gibwork::create_gibwork_task::CreateGibworkTask::new(agent.clone()))
        .dynamic_tool(close_empty_token_accounts::CloseEmptyTokenAccounts::new(agent.clone()))
        .dynamic_tool(stake_with_solayer::StakeWithSolayer::new(agent.clone()))
        .dynamic_tool(get_token_data_by_address::GetTokenData::new());

    builder.build()
}
