use rig::{
    completion::Prompt,
    providers::gemini::{self, completion::GEMINI_1_5_FLASH},
};
use solagent::{fetch_price::FetchPrice, SolAgent};

#[tokio::main]
async fn main() {
    // JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN
    let token_id = "So11111111111111111111111111111111111111112";
    let price = SolAgent::fetch_price(token_id).await.unwrap();
    println!("Price: {}", price);

    let fetch_price_tool = FetchPrice;
    let client = gemini::Client::from_env();
    let agent = client
        .agent(GEMINI_1_5_FLASH)
        .preamble("
            You are an assistant here to help the user select which tool is most appropriate to perform operations.
        ")
        .max_tokens(1024)
        .tool(fetch_price_tool)
        .build();

    // call get balance tool
    let prompt = format!("fetch price of token_id {}", token_id);
    let response = agent
        .prompt(&prompt)
        .await
        .expect("Failed to prompt Gemini");

    println!("Gemini response: {response}");
}
