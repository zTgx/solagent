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

use crate::{actions::get_balance_other, parameters_json_schema, SolanaAgentKit};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct GetBalanceOtherArgs {
    wallet_address: Pubkey,
    token_address: Option<Pubkey>,
}

#[derive(Deserialize, Serialize)]
pub struct GetBalanceOtherOutput {
    pub balance: f64,
}

#[derive(Debug, thiserror::Error)]
#[error("GetBalanceOther error")]
pub struct GetBalanceOtherError;

pub struct GetBalanceOther {
    agent: Arc<SolanaAgentKit>,
}

impl GetBalanceOther {
    pub fn new(agent: Arc<SolanaAgentKit>) -> Self {
        GetBalanceOther { agent }
    }
}

impl Tool for GetBalanceOther {
    const NAME: &'static str = "get_balance_other";

    type Error = GetBalanceOtherError;
    type Args = GetBalanceOtherArgs;
    type Output = GetBalanceOtherOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "get_balance_other".to_string(),
            description: r#"
            
            Get the balance of a Solana wallet or token account which is different from the agent's wallet.
            If no tokenAddress is provided, the SOL balance of the wallet will be returned.
            
            Inputs {
                walletAddress: string, eg "GDEkQF7UMr7RLv1KQKMtm8E2w3iafxJLtyXu3HVQZnME" (required)
                tokenAddress: string, eg "SENDdRQtYMWaQrBroBrJ2Q53fgVuq95CV9UPGEvpCxa" (optional)  
            }
            "#
            .to_string(),
            parameters: parameters_json_schema!(
                wallet_address: String,
                token_address: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let wallet_address = args.wallet_address;
        let token_address = args.token_address;
        let balance = get_balance_other(&self.agent, wallet_address, token_address).await.expect("get_balance_other");

        Ok(GetBalanceOtherOutput { balance })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for GetBalanceOther {
    type InitError = InitError;
    type Context = ();
    type State = Arc<SolanaAgentKit>;

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(GetBalanceOther { agent: _state })
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec![
            "Get the balance of a Solana wallet or token account which is different from the agent's wallet.".into(),
            "If no tokenAddress is provided, the SOL balance of the wallet will be returned.".into(),
        ]
    }

    fn context(&self) -> Self::Context {}
}
