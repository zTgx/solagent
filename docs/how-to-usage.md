# Intro
solagent mainly consists of three parts:

1. `solagent-core`: This is the core library of `solagent.rs`, containing the configuration, Solana wallet, and Solana-related SDKs.

2. `plugins`: These are various functional implementations, such as querying balances, launching tokens, and other API operations. This part can be used in conjunction with `solagent-core`, but some plugins can also be used independently without support from the core, such as `solagent-plugin-goplus`.

3. `tools`: These correspond `one-to-one` with the plugins. The tools are based on the rig framework and can also work with `solagent-core`. To maintain consistency, it is necessary to use the rig exported by `solagent-core`, so the tool must depend on the core.

# Installation
```toml
[dependencies]
solagent-core = "0.1.3"
solagent-rig-goplus = "0.1.0"
```

# Usage
0. Import solana wallet
```rust
let wallet = Wallet::from_env("SOLANA_WALLET_VARIABLE");
```

1. Build config
```rust
let config = ConfigBuilder::default().openai_api_key("test_api_key".to_string()).build();
```

2. New SolanaAgentKit object
```rust
let agent = SolanaAgentKit::new(wallet, "https://api.devnet.solana.com", config);
```

3. Configure your tools for the framework you want to use(currently rig supported only)
```rust
use solagent_rig_goplus::TokenMaliciousInfo;

let tool = TokenMaliciousInfo::new();
```

4. Plug into agent framework
```rust
use solagent_core::{
    rig::{completion::Prompt, providers::openai},
    solana_sdk::signer::keypair::Keypair,
    *,
};

let client = openai::Client::from_url("ollama", 
                "http://localhost:11434/v1");
let agent = client
    .agent(llama3.2)
    .preamble(
        "You are an assistant here to help the user select which tool is most appropriate to perform operations.",
    )
    .tool(tool)
    .build();

let response = agent
    .prompt("check token malicious solana So11111111111111111111111111111111111111112")
    .await
    .expect("Failed to prompt Gemini");

println!("Malicious checking result: {response}");
```


For more examples, please refer to the examples directory.

