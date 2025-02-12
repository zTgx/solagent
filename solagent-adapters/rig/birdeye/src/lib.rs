use anyhow::Result;
use rig_tool_macro::tool;
use solagent_core::{IWallet, SolanaAgentKit};
use solagent_plugin_birdeye::{get_market_data, get_token_overview, MarketDataResponse, TokenOverviewResponse};

#[tool(description = "Get market data of single token by birdeye api")]
pub async fn get_market_data_by_birdeye<W: IWallet>(
    agent: SolanaAgentKit<W>,
    address: String,
) -> Result<GetTransactionResponse> {
    get_market_data(&agent, &address).await
}

#[tool(description = "Get overview of a token by birdeye api")]
pub async fn get_token_overview_by_birdeye<W: IWallet>(
    agent: SolanaAgentKit<W>,
    address: String,
) -> Result<GetTransactionResponse> {
    get_token_overview(&agent, &address).await
}
