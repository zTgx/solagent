pub mod deploy_collection;
pub mod deploy_token;
pub mod fetch_price;
pub mod fetch_price_by_pyth;
pub mod get_balance;
pub mod get_balance_other;
pub mod get_tps;
pub mod get_wallet_address;
pub mod request_faucet_funds;
pub mod transfer;

use super::{
    deploy_token::DeployToken, fetch_price::FetchPrice, fetch_price_by_pyth::FetchPricePyTh,
    get_balance::GetBalance, get_balance_other::GetBalanceOther, get_tps::GetTps,
    get_wallet_address::GetWalletAddress, request_faucet_funds::RequestFaucetFunds,
    transfer::Transfer,
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
