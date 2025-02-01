// Copyright 2025 zTgx
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
#![allow(dead_code)]

//! **solagent.rs: Bridging the Gap Between AI and Solana protocols**  
//! solagent.rs is an open-source Rust library designed to streamline the integration of AI agents with Solana protocols. Built upon the rig framework, solagent.rs empowers developers to construct portable, modular, and lightweight full-stack AI agents capable of interacting with the Solana blockchain.
//!
//! This powerful toolkit simplifies agent-to-blockchain communication, offering a comprehensive suite of functions for tasks such as token operations, trading, and more. By leveraging solagent.rs, developers can seamlessly connect their AI agents to the Solana ecosystem, unlocking a world of possibilities for on-chain automation and intelligent decision-making.
//!
//! Quick Start
//!
//! ```rust
//! use solagent::{Config, SolanaAgentKit, create_solana_tools};
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = Config { openai_api_key: Some("your_api_key".to_string()),
//!             ..Default::default() };
//!     let agent = SolanaAgentKit::new("private_key", "RPC_URL", config);
//!     let toolset = create_solana_tools(agent);
//! }
//! ```
//!
//! Get Balance
//!
//! ```rust
//! use solagent::{Config, SolanaAgentKit};
//!
//! #[tokio::main]
//! async fn main() {
//!    let config = Config { openai_api_key: Some("your_api_key".to_string()),
//!             ..Default::default() };
//!     let agent = SolanaAgentKit::new("private_key", "RPC_URL", config);
//!    let balance = agent.get_balance(None).await.unwrap();
//!    println!("My balance: {}", balance);
//!}
//! ```
//!

mod create_solana_tools;
pub use create_solana_tools;