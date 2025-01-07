/// This example requires that you have the [`ollama`](https://ollama.com) server running locally.
/// More details: https://wale-e.github.io/ai/agent/framework/2025/01/01/hello-world-rig.html
use rig::{completion::Prompt, providers};
use solagent::fetch_price::FetchPrice;

#[tokio::main]
async fn main() -> Result<(), String> {
    let token_id = "So11111111111111111111111111111111111111112";
    let prompt = format!("fetch price of token_id {}", token_id);

    // Create an OpenAI client with a custom base url, a local ollama endpoint
    // The API Key is unnecessary for most local endpoints
    let client = providers::openai::Client::from_url("ollama", "http://localhost:11434/v1");
    // Create agent with a single context prompt
    let comedian_agent = client
        .agent("llama3.2")
        .preamble("You are an assistant here to help the user select which tool is most appropriate to perform operations.")
        .tool(FetchPrice)
        .build();

    // Prompt the agent and print the response
    let response = comedian_agent.prompt(&prompt).await.unwrap();
    println!("{}", response);

    Ok(())
}
