use solagent_core::{
    rig::{
        completion::Prompt,
        providers::gemini::{self, completion::GEMINI_1_5_FLASH},
    },
    *,
};

#[tokio::main]
async fn main() {
    let config = Config { openai_api_key: Some("your_api_key".to_string()), ..Default::default() };
    let agent = SolanaAgentKit::new("private_key", "RPC_URL", config);

    let _v = solagent_plugin_goplus::get_solana_token_security_info("0x").await;

    let _v = solagent_plugin_solana::get_tps(&agent).await;

    let tool = solagent_rig_goplus::TokenMaliciousInfo::new();

    let token_symbol = "SOL";

    let client = gemini::Client::from_env();
    let agent = client
        .agent(GEMINI_1_5_FLASH)
        .preamble(
            "You are an assistant here to help the user select which tool is most appropriate to perform operations.",
        )
        .max_tokens(1024)
        .tool(tool)
        .build();

    let prompt = format!("fetch price of token symbol {}", token_symbol);
    let response = agent.prompt(&prompt).await.expect("Failed to prompt Gemini");

    println!("Gemini response: {response}");
}
