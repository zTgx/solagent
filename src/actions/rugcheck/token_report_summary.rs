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

use crate::primitives::{
    constants::RUGCHECK_URL,
    rugcheck::{Risk, TokenCheck},
};

/// Fetches a summary report for a specific token.
///
/// # Parameters
///
/// - `mint` - The mint address of the token.
///
/// # Returns
/// Token summary report.
///
/// # Errors
/// Throws an error if the API call fails.
pub async fn fetch_summary_report(mint: String) -> Result<TokenCheck, Box<dyn std::error::Error>> {
    let url = format!("{}/tokens/{}/report/summary", RUGCHECK_URL, mint);

    let response = reqwest::get(&url).await?;
    if !response.status().is_success() {
        return Err(format!("HTTP error! status: {}", response.status()).into());
    }

    let data: serde_json::Value = response.json().await?;

    let mut token_check = TokenCheck::default();
    let token_program = data.get("tokenProgram").and_then(|p| p.as_str()).expect("tokenProgram field");
    token_check.token_program = token_program.into();

    let token_type = data.get("tokenType").and_then(|p| p.as_str()).expect("tokenType field");
    token_check.token_type = token_type.into();

    let mut risks: Vec<Risk> = vec![];
    let risks_data = data.get("risks").and_then(|p| p.as_array()).expect("risks field");
    for risk in risks_data {
        let mut r = Risk::default();
        let name = risk.get("name").and_then(|p| p.as_str()).expect("name field");
        let description = risk.get("description").and_then(|p| p.as_str()).expect("description field");
        let score = risk.get("score").and_then(|p| p.as_f64()).expect("score field");
        let level = risk.get("level").and_then(|p| p.as_str()).expect("level field");

        r.name = name.into();
        r.description = description.into();
        r.score = score;
        r.level = level.into();

        risks.push(r);
    }
    token_check.risks = risks;

    Ok(token_check)
}
