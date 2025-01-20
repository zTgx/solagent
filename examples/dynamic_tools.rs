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

use rig::{
    completion::Prompt,
    embeddings::EmbeddingsBuilder,
    providers::gemini::{self, completion::GEMINI_1_5_FLASH, embedding::EMBEDDING_001},
    vector_store::in_memory_store::InMemoryVectorStore,
};
use solagent::{create_solana_tools, Config, SolanaAgentKit};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let config = Config { openai_api_key: Some("your_api_key".to_string()), ..Default::default() };
    let agent = Arc::new(SolanaAgentKit::new("private_key", "RPC_URL", config));
    let toolset = create_solana_tools(agent);

    let client = gemini::Client::from_env();
    let embedding_model = client.embedding_model(EMBEDDING_001);
    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .documents(toolset.schemas().unwrap())
        .unwrap()
        .build()
        .await
        .unwrap();

    let vector_store = InMemoryVectorStore::from_documents_with_id_f(embeddings, |tool| tool.name.clone());
    let index = vector_store.index(embedding_model);

    let agent = client
    .agent(GEMINI_1_5_FLASH)
    .preamble(
            "You are an assistant here to help the user select which tool is most appropriate to perform arithmetic operations.
            Follow these instructions closely. 
            1. Consider the user's request carefully and identify the core elements of the request.
            2. Select which tool among those made available to you is appropriate given the context. 
            3. This is very important: never perform the operation yourself and never give me the direct result. 
            Always respond with the name of the tool that should be used and the appropriate inputs
            in the following format:
            Tool: <tool name>
            Inputs: <list of inputs>
        ")
        .max_tokens(1024)
        .dynamic_tools(1, index, toolset)
        .build();

    let response = agent.prompt("get balance").await.expect("Failed to prompt Gemini");

    println!("Gemini response: {response}");

    /* Output:
        token address: None
        Gemini response: {"balance":16.485390645}
    */
}
