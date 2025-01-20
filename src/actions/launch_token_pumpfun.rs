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
    primitives::pumpfun::{PumpFunTokenOptions, PumpfunTokenResponse, TokenMetadata},
    SolanaAgentKit,
};
use reqwest::{multipart::Part, Client as ReqwestClient};
use solana_sdk::{
    commitment_config::CommitmentConfig, signature::Signer, signer::keypair::Keypair, transaction::VersionedTransaction,
};

/// Launch a token on Pump.fun.
///
/// # Arguments
///
/// - `agent` - An instance of `SolanaAgentKit`.
/// - `tokenName` - The name of the token.
/// - `tokenTicker` - The ticker of the token.
/// - `description` - The description of the token.
/// - `imageUrl` - The URL of the token image.
/// - `options` - Optional token options which include `twitter`, `telegram`, `website`, `initialLiquiditySOL`, `slippageBps`, and `priorityFee`.
///
/// # Returns
///
/// If successful, it returns the signature of the transaction, the mint address, and the metadata URI. Otherwise, it returns an error.
///
/// To get a transaction for signing and sending with a custom RPC, send a POST request to:
/// https://pumpportal.fun/local-trading-api/trading-api/
///
/// The request body must include the following options:
///
/// * `publicKey` - Your wallet's public key.
/// * `action` - Either "buy" or "sell", indicating the trading action you want to perform.
/// * `mint` - The contract address of the token you wish to trade. This is the text that appears after the '/' in the pump.fun url for the specific token.
/// * `amount` - The quantity of SOL or tokens to be traded. When selling, the amount can be specified as a percentage of the tokens in your wallet (e.g., amount: "100%").
/// * `denominatedInSol` - Set to "true" if the `amount` is specified in SOL, and "false" if it's specified in tokens.
/// * `slippage` - The percentage of slippage that is allowed during the trading process.
/// * `priorityFee` - The amount to be used as the priority fee.
/// * `pool` - (Optional) Currently, 'pump' and 'raydium' are the supported options. The default value is 'pump'.
///
///
/// https://pumpportal.fun/creation
///
pub async fn launch_token_pumpfun(
    agent: &SolanaAgentKit,
    token_name: &str,
    token_symbol: &str,
    description: &str,
    image_url: &str,
    options: Option<PumpFunTokenOptions>,
) -> Result<PumpfunTokenResponse, Box<dyn std::error::Error>> {
    let reqwest_client = ReqwestClient::new();

    // 0. download image
    let image_data = fetch_image(&reqwest_client, image_url).await.expect("fetch_image");

    // 1. fetch token metadata metadataUri
    let token_metadata =
        fetch_token_metadata(&reqwest_client, token_name, token_symbol, description, options, &image_data)
            .await
            .expect("fetch_token_metadata");

    // 2. Create a new keypair for the mint
    let mint_keypair = Keypair::new();

    // 3. request pumpportal tx
    let mut versioned_tx = request_pumpportal_tx(agent, &reqwest_client, &token_metadata, &mint_keypair)
        .await
        .expect("request_pumpportal_tx");

    // 4. sign&send transaction
    let signature = sign_and_send_tx(agent, &mut versioned_tx, &mint_keypair).await.expect("sign_and_send_tx");

    let res =
        PumpfunTokenResponse { signature, mint: mint_keypair.pubkey().to_string(), metadata_uri: token_metadata.uri };

    Ok(res)
}

// try signed vtx: NotEnoughSigners -> mint_keypair is needed
async fn sign_and_send_tx(
    agent: &SolanaAgentKit,
    vtx: &mut VersionedTransaction,
    mint_keypair: &Keypair,
) -> Result<String, Box<std::io::Error>> {
    let recent_blockhash = agent.connection.get_latest_blockhash().expect("get_latest_blockhash");
    vtx.message.set_recent_blockhash(recent_blockhash);
    let signed_vtx = VersionedTransaction::try_new(vtx.message.clone(), &[mint_keypair, &agent.wallet.wallet])
        .expect("try signed vtx");

    let signature = agent
        .connection
        .send_and_confirm_transaction_with_spinner_and_config(
            &signed_vtx,
            CommitmentConfig::finalized(),
            solana_client::rpc_config::RpcSendTransactionConfig { skip_preflight: false, ..Default::default() },
        )
        .expect("send_and_confirm_tx");

    Ok(signature.to_string())
}

async fn fetch_image(client: &ReqwestClient, image_url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let response = client.get(image_url).send().await?;
    if response.status().is_success() {
        let image_data = response.bytes().await.expect("image data");
        return Ok(image_data.to_vec());
    }

    Err("fetch image error".into())
}

async fn fetch_token_metadata(
    client: &ReqwestClient,
    name: &str,
    symbol: &str,
    description: &str,
    options: Option<PumpFunTokenOptions>,
    image_data: &[u8],
) -> Result<TokenMetadata, Box<dyn std::error::Error>> {
    let part = Part::bytes(image_data.to_vec()).file_name("image_name").mime_str("image/png")?; // Important: set the correct MIME type

    let mut form = reqwest::multipart::Form::new()
        .text("name", name.to_owned())
        .text("symbol", symbol.to_owned())
        .text("description", description.to_owned())
        .part("file", part);

    if let Some(option) = options {
        if let Some(x) = option.twitter {
            form = form.text("twitter", x);
        }

        if let Some(tele) = option.telegram {
            form = form.text("telegram", tele);
        }

        if let Some(website) = option.website {
            form = form.text("website", website);
        }

        form = form.text("showName", "true");
    }

    let res = client.post("https://pump.fun/api/ipfs").multipart(form).send().await?;

    let status = res.status();
    if !status.is_success() {
        let text = res.text().await?;
        eprintln!("Error response: {}", text);
        return Err(format!("Upload failed with status: {}", status).into());
    }

    let response_json = res.json::<serde_json::Value>().await?;
    let md = TokenMetadata {
        name: name.to_string(),
        symbol: symbol.to_string(),
        uri: response_json.get("metadataUri").unwrap().to_string(),
    };

    Ok(md)
}

async fn request_pumpportal_tx(
    agent: &SolanaAgentKit,
    client: &ReqwestClient,
    token_matedata: &TokenMetadata,
    mint_keypair: &Keypair,
) -> Result<VersionedTransaction, Box<dyn std::error::Error>> {
    let request_body = serde_json::json!({
        "publicKey": agent.wallet.address.to_string(),
        "action": "create",
        "tokenMetadata": {
            "name": token_matedata.name,
            "symbol": token_matedata.symbol,
            "uri": token_matedata.uri
        },
        "mint": mint_keypair.pubkey().to_string(),
        "denominatedInSol": "true",
        "amount": 1,
        "slippage": 10,
        "priorityFee": 0.0005,
        "pool": "pump"
    });

    let res = client
        .post("https://pumpportal.fun/api/trade-local")
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    let status = res.status();
    if !status.is_success() {
        let text = res.text().await?;
        eprintln!("Error response: {}", text);
        return Err(format!("trade-local failed with status: {}", status).into());
    }

    let bytes = res.bytes().await.unwrap();
    let tx: VersionedTransaction = bincode::deserialize(&bytes).unwrap();
    Ok(tx)
}
