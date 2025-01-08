use rig::{
    completion::Prompt,
    embeddings::EmbeddingsBuilder,
    providers::gemini::{self, completion::GEMINI_1_5_FLASH, embedding::EMBEDDING_001},
    vector_store::in_memory_store::InMemoryVectorStore,
};
use solagent::create_solana_tools;

#[tokio::main]
async fn main() {
    let client = gemini::Client::from_env();

    let toolset = create_solana_tools();

    let embedding_model = client.embedding_model(EMBEDDING_001);
    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .documents(toolset.schemas().unwrap())
        .unwrap()
        .build()
        .await
        .unwrap();

    let vector_store =
        InMemoryVectorStore::from_documents_with_id_f(embeddings, |tool| tool.name.clone());
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

    let response = agent
        .prompt("get balance")
        .await
        .expect("Failed to prompt Gemini");

    println!("Gemini response: {response}");

    /* Output:
        token address: None
        Gemini response: {"balance":16.485390645}
    */
}
