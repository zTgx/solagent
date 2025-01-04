use rig::{
    completion::Prompt,
    providers::gemini::{self, completion::GEMINI_1_5_FLASH},
};
use solagent::{tools::get_tps::GetTps, SOL_AGENT};

#[tokio::main]
async fn main() {
    let tool = GetTps::new(&SOL_AGENT);

    let client = gemini::Client::from_env();
    let agent = client
        .agent(GEMINI_1_5_FLASH)
        .preamble("
            You are an assistant here to help the user select which tool is most appropriate to perform operations.
        ")
        .max_tokens(1024)
        .tool(tool)
        .build();

    let response = agent
        .prompt("Get TPS")
        .await
        .expect("Failed to prompt Gemini");

    println!("Gemini response: {response}");
}
