use rig::{
    completion::Prompt,
    providers::gemini::{self, completion::GEMINI_1_5_FLASH},
};
use solagent::{get_tps::GetTps, SOL_AGENT};

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

    // Demo : how to use create_solana_tools
    // {
    //     let model = client.completion_model(GEMINI_1_5_FLASH);

    //     let mut builder = AgentBuilder::new(model).preamble("System prompt");

    //     let toolset = create_solana_tools(&SOL_AGENT);

    //     for tool in toolset {
    //         match tool {
    //             solagent::tools::ToolSet::GetBalance(t) => {
    //                 builder = builder.tool(t);
    //             }
    //             _ => todo!(),
    //         }
    //     }

    //     let agent = builder.temperature(0.8).build();

    //     let response = agent
    //         .prompt("Get TPS")
    //         .await
    //         .expect("Failed to prompt Gemini");

    //     println!("Gemini response: {response}");
    // }

    let response = agent
        .prompt("Get TPS")
        .await
        .expect("Failed to prompt Gemini");

    println!("Gemini response: {response}");
}
