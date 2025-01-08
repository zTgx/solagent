use super::{
    get_balance::GetBalance, get_balance_other::GetBalanceOther,
    request_faucet_funds::RequestFaucetFunds,
};
use crate::{agent::SolAgent, SOL_AGENT};
use rig::tool::ToolSet as RigToolSet;

/// An enumeration representing a set of tools that can be used with Solana.
///
/// This enum encapsulates different tool types, including:
/// - `GetBalance`: Tool to get the balance of a specified wallet.
/// - `GetBalanceOther`: Tool to get the balance of another wallet.
/// - `RequestFaucetFunds`: Tool to request funds from a faucet.
///
/// The use of an enum here is necessary because the `Tool` trait is not object-safe,
/// meaning it cannot be used as a dynamic trait object (`dyn Tool`). This is due to
/// the requirement that the trait must be `Sized`, which prevents it from being
/// treated as a trait object. By using an enum, we can define a fixed set of tools
/// at compile time, allowing us to leverage Rust's type system while avoiding
/// dynamic dispatch.
pub enum ToolSet<'a> {
    GetBalance(GetBalance<'a>),
    GetBalanceOther(GetBalanceOther<'a>),
    RequestFaucetFunds(RequestFaucetFunds<'a>),
}

/// Creates a vector of tools for interacting with Solana.
///
/// # Parameters
///
/// - `solagent`: A reference to an instance of `SolAgent`. This is used to create
///   instances of various tools that require access to the Solana agent.
///
/// # Returns
///
/// A vector containing instances of different tools wrapped in the `ToolSet` enum.
pub fn create_solana_tools<'a>(solagent: &'a SolAgent) -> Vec<ToolSet<'a>> {
    vec![
        ToolSet::GetBalance(GetBalance::new(solagent)),
        ToolSet::GetBalanceOther(GetBalanceOther::new(solagent)),
        ToolSet::RequestFaucetFunds(RequestFaucetFunds::new(solagent)),
    ]
}

pub fn create_solana_toolx() -> RigToolSet {
    let mut builder = RigToolSet::builder();
    builder = builder.dynamic_tool(GetBalance::new(&SOL_AGENT));
    // builder = builder.static_tool(GetBalanceOther::new(&SOL_AGENT));
    // builder = builder.static_tool(RequestFaucetFunds::new(&SOL_AGENT));

    builder.build()
}
