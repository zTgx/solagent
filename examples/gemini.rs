use rig::{
    completion::Prompt,
    providers::gemini::{self, completion::GEMINI_1_5_FLASH},
};
use solagent::{tools::get_balance::GetBalance, SOL_AGENT};

#[tokio::main]
async fn main() {
    let get_balance_tool = GetBalance::new(&SOL_AGENT);

    let client = gemini::Client::from_env();
    let agent = client
        .agent(GEMINI_1_5_FLASH)
        .preamble("
            You are an assistant here to help the user select which tool is most appropriate to perform operations.
        ")
        .max_tokens(1024)
        .tool(get_balance_tool)
        .build();

    let response = agent
        .prompt("Get balance")
        .await
        .expect("Failed to prompt Gemini");

    println!("Gemini response: {response}");
}
