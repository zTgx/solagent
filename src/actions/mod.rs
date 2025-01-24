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

mod get_balance_other;
pub use get_balance_other::get_balance_other;

mod deploy_token;
pub use deploy_token::deploy_token;

mod deploy_collection;
pub use deploy_collection::deploy_collection;

mod fetch_price;
pub use fetch_price::fetch_price;

mod pyth_fetch_price;
pub use pyth_fetch_price::{fetch_price_by_pyth, fetch_pyth_price_feed_id};

mod get_wallet_address;
pub use get_wallet_address::get_wallet_address;

mod mint_nft;
pub use mint_nft::mint_nft_to_collection;

mod launch_token_pumpfun;
pub use launch_token_pumpfun::launch_token_pumpfun;

mod trade;
pub use trade::trade;

mod stake_with_jup;
pub use stake_with_jup::stake_with_jup;

mod rugcheck;
pub use rugcheck::{fetch_detailed_report, fetch_summary_report};

mod create_gibwork_task;
pub use create_gibwork_task::{create_gibwork_task, GibworkCreateTaskResponse};

mod solana;
pub use solana::{close_empty_token_accounts, get_balance, get_tps, request_faucet_funds, transfer};

mod solayer;
pub use solayer::stake_with_solayer;

mod helius;
pub use helius::{create_webhook, HeliusWebhookResponse};

mod jupiter;
pub use jupiter::get_token_data_by_address;
