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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PumpFunTokenOptions {
    pub twitter: Option<String>,
    pub telegram: Option<String>,
    pub website: Option<String>,
    pub initial_liquidity_sol: f64,
    pub slippage_bps: u16,
    pub priority_fee: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PumpfunTokenResponse {
    pub signature: String,
    pub mint: String,
    pub metadata_uri: String,
}

pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
