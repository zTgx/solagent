use rig::{
    completion::Prompt,
    providers::gemini::{self, completion::GEMINI_1_5_FLASH},
};
use solagent::fetch_price_by_pyth::FetchPricePyTh;

#[tokio::main]
async fn main() {
    let price_feed_id = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";

    let fetch_price_tool = FetchPricePyTh;
    let client = gemini::Client::from_env();
    let agent = client
        .agent(GEMINI_1_5_FLASH)
        .preamble("You are an assistant here to help the user select which tool is most appropriate to perform operations.")
        .max_tokens(1024)
        .tool(fetch_price_tool)
        .build();

    let prompt = format!("fetch price of price_feed_id {}", price_feed_id);
    let response = agent
        .prompt(&prompt)
        .await
        .expect("Failed to prompt Gemini");

    println!("Gemini response: {response}");
}
