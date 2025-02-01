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

mod get_token_data_by_address;
pub use get_token_data_by_address::get_token_data_by_address;

mod trade;
pub use trade::trade;

mod fetch_price;
pub use fetch_price::fetch_price;

mod stake_with_jup;
pub use stake_with_jup::stake_with_jup;

/// Jupiter API URL
pub const JUP_API: &str = "https://quote-api.jup.ag/v6";
pub const JUP_REFERRAL_ADDRESS: &str = "REFER4ZgmyYx9c6He5XfaTMiGfdLwRnkV4RPp9t9iF3";
pub const JUP_PRICE_V2: &str = "https://api.jup.ag/price/v2?ids=";
