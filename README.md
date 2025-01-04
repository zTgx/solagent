# solagent.rs   
  [<img alt="crates.io" src="https://img.shields.io/crates/v/solagent?style=for-the-badge&logo=rust">](https://crates.io/crates/solagent)
  [<img alt="docs.rs" src="https://img.shields.io/docsrs/solagent?style=for-the-badge&logo=docs.rs">](https://docs.rs/solagent)
 ![Crates Downloads](https://img.shields.io/crates/d/solagent?style=for-the-badge&logo=rust)
  

connect any Ai agents to solana protocols in Rust.

## 📦 Installation

```bash
[dependencies]
solagent = "0.1.2"
```

## Quick Start
```shell
cp exampel.config.toml config.toml
```
```rust
#[tokio::main]
async fn main() {
    let get_balance_tool = GetBalance::new(&SOL_AGENT);

    let client = gemini::Client::from_env();
    let agent = client
        .agent(GEMINI_1_5_FLASH)
        .preamble("
            You are an assistant here to help the user select which tool is most appropriate to perform operations.
        ")
        .max_tokens(1024)
        .tool(get_balance_tool)
        .build();

    let response = agent
        .prompt("Get balance")
        .await
        .expect("Failed to prompt Gemini");

    println!("Gemini response: {response}");
}
```
