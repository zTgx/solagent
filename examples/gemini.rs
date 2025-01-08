use rig::{
    completion::Prompt,
    providers::gemini::{self, completion::GEMINI_1_5_FLASH},
};
use solagent::get_balance::GetBalance;

#[tokio::main]
async fn main() {
    let balance_tool = GetBalance::new();

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

    /* Output:
        token address: None
        Gemini response: {"balance":16.485390645}
    */
}
