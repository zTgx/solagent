use rig::{
    completion::Prompt,
    providers::gemini::{self, completion::GEMINI_1_5_FLASH},
};
use solagent::{get_balance::GetBalance, SolAgent};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let agent = Arc::new(SolAgent::new(
        "private_key",
        "https://api.devnet.solana.com",
        "openai_api_key",
    ));
    let balance_tool = GetBalance::new(agent);

    let client = gemini::Client::from_env();
    let agent = client
        .agent(GEMINI_1_5_FLASH)
        .preamble("You are an assistant here to help the user select which tool is most appropriate to perform operations.")
        .max_tokens(1024)
        .tool(balance_tool)
        .build();

    // call get balance tool
    let response = agent
        .prompt("get balance")
        .await
        .expect("Failed to prompt Gemini");

    println!("Gemini response: {response}");

    drop(agent)
    /* Output:
        token address: None
        Gemini response: {"balance":16.485390645}
    */
}
