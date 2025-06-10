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

pub async fn get_token_security_info(
    chain_id: &str,
    contract_address: &str,
) -> Result<serde_json::Value, reqwest::Error> {
    let url =
        format!("https://api.gopluslabs.io/api/v1/token_security/{}?contract_addresses={}", chain_id, contract_address);

    let client = reqwest::Client::new();
    let response = client.get(url).header("Content-Type", "application/json").send().await?;

    let data = response.json::<serde_json::Value>().await?;
    Ok(data)
}

pub async fn get_solana_token_security_info(contract_address: &str) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("https://api.gopluslabs.io/api/v1/solana/token_security?contract_addresses={}", contract_address);

    let client = reqwest::Client::new();
    let response = client.get(url).header("Content-Type", "application/json").send().await?;

    let data = response.json::<serde_json::Value>().await?;
    Ok(data)
}

pub async fn get_token_malicious_info(chain_id: &str, address: &str) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("https://api.gopluslabs.io/api/v1/address_security/{}?chain_id={}", address, chain_id);

    let client = reqwest::Client::new();
    let response = client.get(url).header("Content-Type", "application/json").send().await?;

    let data = response.json::<serde_json::Value>().await?;
    Ok(data)
}

pub async fn get_token_phishing_site_info(url: &str) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("https://api.gopluslabs.io/api/v1/aphishing_site?url={}", url);

    let client = reqwest::Client::new();
    let response = client.get(url).header("Content-Type", "application/json").send().await?;

    let data = response.json::<serde_json::Value>().await?;
    Ok(data)
}
