# How to Add a Tool to SolanaAgentKit

## Introduction
This guide provides step-by-step instructions for new builders on how to add a new tool workflow to the SolanaAgentKit project. By following these steps, you can easily integrate new functionalities into the toolset.

## Directory Structure
The tools are organized in the `src/tools` directory. Each tool is defined in its own Rust file, and the main entry point for the tools is located in `mod.rs`.

## First Steps : Implementing a New Action
1. **Implement the functionality in action directory**:
   - Create the action file in `src/actions/my_new_action.rs`.
   - Implement the action logic in the action file.
   - Import the action module in the `mod.rs` file in the `src/actions` directory.
2. **Implement the agent implementation**:
  - Create the same name of function in `src/agents/agent_impl.rs`.
  - Call the action function in the agent implementation.

## Second Steps : Creating a New Tool
1. **Create a New Tool File**: 
   - Navigate to the `src/tools` directory.
   - Create a new Rust file for your tool, e.g., `my_new_tool.rs`.

2. **Define Your Tool**:
   - In your new file, define your tool structure and implement the necessary methods. Ensure you import any required modules.

   Example structure:
  ```rust
  pub struct FetchPrice;
  impl FetchPrice {
      pub fn new() -> Self {
          FetchPrice {}
      }
  }

  impl Tool for FetchPrice {
      const NAME: &'static str = "fetch_price";

      type Error = FetchPriceError;
      type Args = FetchPriceArgs;
      type Output = FetchPriceOutput;

      async fn definition(&self, _prompt: String) -> ToolDefinition {
          ToolDefinition {
              name: "fetch_price".to_string(),
              description: r#"Fetch the current price of a Solana token in USDC using Jupiter API
                  input: {
                      token_id: "",
                  },
              "#
              .to_string(),
              parameters: parameters_json_schema!(
                  token_id: String,
              ),
          }
      }

      async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
          let token_id = args.token_id;
          let price = fetch_price(&token_id).await.expect("fetch_price");

          Ok(FetchPriceOutput { price })
      }
  }

  #[derive(Debug, thiserror::Error)]
  #[error("Init error")]
  pub struct InitError;

  impl ToolEmbedding for FetchPrice {
      type InitError = InitError;
      type Context = ();
      type State = ();

      fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
          Ok(FetchPrice {})
      }

      fn embedding_docs(&self) -> Vec<String> {
          vec!["Fetch the current price of a Solana token in USDC using Jupiter API".into()]
      }

      fn context(&self) -> Self::Context {}
  }
   ```

3. **Register the Tool**:
   - In the `mod.rs` file in the `src/tools` directory, add the following line:
     ```rust
     pub mod my_new_tool;
     ```
   - Import the tool in the `mod.rs` file:
     ```rust
     use crate::tools::my_new_tool::MyNewTool;
     ```

   Example structure:
   ```rust
   use crate::tools::my_new_tool::MyNewTool;
   ```   

4. **Implement the Tool Definition**:  
5. **Implement the Tool Embedding**:     
6. **Implement the Tool**:
7. **Add examples**: 

## Conclusion
By following these steps, you have successfully added a new tool to the SolanaAgentKit project. You can now use the tool in your workflows and enhance the capabilities of the SolanaAgentKit project.