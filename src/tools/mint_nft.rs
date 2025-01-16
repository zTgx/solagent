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

use crate::{
    primitives::token::NFTMetadata,
    SolanaAgentKit,
    {actions::mint_nft_to_collection, parameters_json_schema},
};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MintNFTArgs {
    collection: Pubkey,
    metadata: NFTMetadata,
}

#[derive(Deserialize, Serialize)]
pub struct MintNFTOutput {
    pub mint_address: String,
    pub tx_signature: String,
}

#[derive(Debug, thiserror::Error)]
#[error("MintNFT error")]
pub struct MintNFTError;

pub struct MintNFT {
    agent: Arc<SolanaAgentKit>,
}

impl MintNFT {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        MintNFT { agent }
    }
}

impl Tool for MintNFT {
    const NAME: &'static str = "mint_nft";

    type Error = MintNFTError;
    type Args = MintNFTArgs;
    type Output = MintNFTOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "mint_nft".to_string(),
            description: r#"
            Mint a new NFT in a collection on Solana blockchain.

            examples: [
                [
                    {
                        input: {
                            collection: "J1S9H3QjnRtBbbuD4HjPV6RpRhwuk4zKbxsnCHuTgh9w",
                            metadata: {
                                name: "My NFT",
                                uri: "https://example.com/nft.json",
                                basis_points: Option<u16>,
                            }
                        },
                        output: {
                            status: "success",
                            message: "NFT minted successfully",
                            mintAddress: "7nE9GvcwsqzYxmJLSrYmSB1V1YoJWVK1KWzAcWAzjXkN",
                            metadata: {
                                name: "My NFT",
                                uri: "https://example.com/nft.json",
                            },
                            recipient: "7nE9GvcwsqzYxmJLSrYmSB1V1YoJWVK1KWzAcWAzjXkN",
                        },
                        explanation: "Mint an NFT to the default wallet",
                    },
                ],
                [
                    {
                        input: {
                            collection: "J1S9H3QjnRtBbbuD4HjPV6RpRhwuk4zKbxsnCHuTgh9w",
                            metadata: {
                                name: "My NFT",
                                uri: "https://example.com/nft.json",
                                basis_points: Option<u16>,
                                creators: [
                                    address: "address",
                                    verified: false,
                                    share: 100,
                                ]                        
                            }                        
                        },
                        output: {
                            status: "success",
                            message: "NFT minted successfully",
                            mintAddress: "8nE9GvcwsqzYxmJLSrYmSB1V1YoJWVK1KWzAcWAzjXkM",
                            metadata: {
                                name: "Gift NFT",
                                uri: "https://example.com/gift.json",
                            },
                            recipient: "9aUn5swQzUTRanaaTwmszxiv89cvFwUCjEBv1vZCoT1u",
                        },
                        explanation: "Mint an NFT to a specific recipient",
                    },
                ],
            ],

            "#
            .to_string(),
            parameters: parameters_json_schema!(
                collection: Pubkey,
                metadata: NFTMetadata,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let res = mint_nft_to_collection(&self.agent, args.collection, args.metadata).await.expect("mint_nft");

        Ok(MintNFTOutput { mint_address: res.mint, tx_signature: res.signature })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for MintNFT {
    type InitError = InitError;
    type Context = ();
    type State = Arc<SolanaAgentKit>;

    fn init(state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(MintNFT { agent: state })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Mint a new NFT in a collection on Solana blockchain.".into()]
    }

    fn context(&self) -> Self::Context {}
}
