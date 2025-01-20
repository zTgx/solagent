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

use crate::{actions::fetch_detailed_report, parameters_json_schema};
use rig::{
    completion::ToolDefinition,
    tool::{Tool, ToolEmbedding},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct FetchTokenReportDetailedArgs {
    mint: String,
}

#[derive(Deserialize, Serialize)]
pub struct FetchTokenReportDetailedOutput {
    pub token_check: Value,
}

#[derive(Debug, thiserror::Error)]
#[error("FetchTokenReportDetailed error")]
pub struct FetchTokenReportDetailedError;

#[derive(Default)]
pub struct FetchTokenReportDetailed {}

impl FetchTokenReportDetailed {
    pub fn new() -> Self {
        FetchTokenReportDetailed {}
    }
}

impl Tool for FetchTokenReportDetailed {
    const NAME: &'static str = "fetch_detailed_report";

    type Error = FetchTokenReportDetailedError;
    type Args = FetchTokenReportDetailedArgs;
    type Output = FetchTokenReportDetailedOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "fetch_detailed_report".to_string(),
            description: r#"
                Fetches a detailed report for a specific token from RugCheck.
                Inputs:
                - mint: string, the mint address of the token, e.g., "84VUXykQjNvPDm88oT5FRucXeNcrwdQGottJKjkAoqd1" (required).
            "#.to_string(),
            parameters: parameters_json_schema!(
                mint: String,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let token_check = fetch_detailed_report(args.mint).await.expect("fetch_detailed_report");
        Ok(FetchTokenReportDetailedOutput { token_check })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Init error")]
pub struct InitError;

impl ToolEmbedding for FetchTokenReportDetailed {
    type InitError = InitError;
    type Context = ();
    type State = ();

    fn init(_state: Self::State, _context: Self::Context) -> Result<Self, Self::InitError> {
        Ok(FetchTokenReportDetailed {})
    }

    fn embedding_docs(&self) -> Vec<String> {
        vec!["Fetches a summary report for a specific token from FetchTokenReportDetailed.".into()]
    }

    fn context(&self) -> Self::Context {}
}
