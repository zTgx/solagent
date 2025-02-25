use std::{env, sync::Arc};

use anyhow::Result;
use solagent_core::{
    rig::{
        agent::Agent,
        providers::{self, openai::CompletionModel},
    },
    ConfigBuilder, SolanaAgentKit,
};
use solagent_rig_solana::{
    close_empty_token_accounts::CloseEmptyTokenAccounts, get_balance::GetBalance, get_tps::GetTps,
};
use solagent_wallet_solana::Wallet;

pub fn create_solana_agent() -> Result<Agent<CompletionModel>> {
    let wallet = Wallet::from_env("SOLANA_PRIVATE_KEY").expect("SOLANA_PRIVATE_KEY");
    let rpc_url = env::var("key").expect("SOLANA_RPC_URL");
    let config = ConfigBuilder::default().build();

    let solana_agent_kit = SolanaAgentKit::new(wallet, &rpc_url, config);
    let solana_agent_kit = Arc::new(solana_agent_kit);

    let close_acc = CloseEmptyTokenAccounts::new(solana_agent_kit.clone());
    let balance = GetBalance::new(solana_agent_kit.clone());
    let tps = GetTps::new(solana_agent_kit.clone());

    let client = providers::openai::Client::from_url("ollama", "http://localhost:11434/v1");
    let agent = client
        .agent("llama3.2")
        .preamble("You are an solana trading agent here to help the user.")
        .max_tokens(1024)
        .tool(close_acc)
        .tool(balance)
        .tool(tps)
        .build();

    Ok(agent)
}
