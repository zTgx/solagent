use rig::{completion::Prompt, providers::openai};
use solagent::tools::get_balance::GetBalance;

#[tokio::main]
async fn main() {
    let agent = solagent::agent::SolAgent::new();
    let get_balance_tool = GetBalance::new(agent);

    let openai_client = openai::Client::from_env();
    let agent = openai_client
        .agent("gpt-4")
        .preamble("
            You are an assistant here to help the user select which tool is most appropriate to perform operations.
        ")
        .max_tokens(1024)
        .tool(get_balance_tool)
        .build();

    let response = agent
        .prompt("Get balance")
        .await
        .expect("Failed to prompt GPT-4");

    println!("GPT-4: {response}");
}
