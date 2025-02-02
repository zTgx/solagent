use solagent_core::{
    rig::{
        completion::Prompt,
        providers::gemini::{self, completion::GEMINI_1_5_FLASH},
    },
    solana_sdk::signer::keypair::Keypair,
    *,
};

#[tokio::main]
async fn main() {
    // Create a new keypair
    let keypair = Keypair::new();
    // Encode the secret key to base58
    let private_key = keypair.to_base58_string();

    let config = Config { cookie_api_key: Some("".to_string()), ..Default::default() };
    let agent = SolanaAgentKit::new(&private_key, "https://api.devnet.solana.com", config);

    let _v = solagent_plugin_cookie::search_tweets(&agent, "ztgx5", "2025-01-01", "2025-01-20").await.unwrap();
    let _v =
        solagent_plugin_goplus::get_solana_token_security_info("So11111111111111111111111111111111111111112").await;

    let _v = solagent_plugin_solana::get_tps(&agent).await;

    let tool = solagent_rig_goplus::TokenMaliciousInfo::new();

    let client = gemini::Client::from_env();
    let agent = client
        .agent(GEMINI_1_5_FLASH)
        .preamble(
            "You are an assistant here to help the user select which tool is most appropriate to perform operations.",
        )
        .max_tokens(1024)
        .tool(tool)
        .build();

    let response = agent
        .prompt("check token malicious solana So11111111111111111111111111111111111111112")
        .await
        .expect("Failed to prompt Gemini");

    println!("Gemini response: {response}");
}
