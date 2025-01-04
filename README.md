# solagent.rs   
  [<img alt="crates.io" src="https://img.shields.io/crates/v/solagent?style=for-the-badge&logo=rust">](https://crates.io/crates/solagent)
  [<img alt="docs.rs" src="https://img.shields.io/docsrs/solagent?style=for-the-badge&logo=docs.rs">](https://docs.rs/solagent)

connect any Ai agents to solana protocols in Rust.

## 📦 Installation

```bash
[dependencies]
solagent = "0.1.1"
```

## Quick Start
```shell
cp exampel.config.toml config.toml
```
```rust
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
```
