#![allow(dead_code)]

use lazy_static::lazy_static;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

lazy_static! {
    /// Common token addresses used across the toolkit
    pub static ref TOKENS: HashMap<&'static str, Pubkey> = {
        let mut m = HashMap::new();
        m.insert("USDC", Pubkey::from_str_const("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"));
        m.insert("USDT", Pubkey::from_str_const("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB"));
        m.insert("USDS", Pubkey::from_str_const("USDSwr9ApdHk5bvJKMjzff41FfuX8bSxdKcR81vTwcA"));
        m.insert("SOL", Pubkey::from_str_const("So11111111111111111111111111111111111111112"));
        m.insert("jitoSOL", Pubkey::from_str_const("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn"));
        m.insert("bSOL", Pubkey::from_str_const("bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1"));
        m.insert("mSOL", Pubkey::from_str_const("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So"));
        m.insert("BONK", Pubkey::from_str_const("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"));
        m
    };
}

/// Default configuration options
pub const DEFAULT_OPTIONS: Options = Options {
    slippage_bps: 300,
    token_decimals: 9,
    referral_fee: 200,
};

#[derive(Debug)]
pub struct Options {
    pub slippage_bps: u32, // Default slippage tolerance in basis points (300 = 3%)
    pub token_decimals: u32, // Default number of decimals for new tokens
    pub referral_fee: u32, // Referral fee
}

/// Jupiter API URL
pub const JUP_API: &str = "https://quote-api.jup.ag/v6";
pub const JUP_REFERRAL_ADDRESS: &str = "REFER4ZgmyYx9c6He5XfaTMiGfdLwRnkV4RPp9t9iF3";
pub const JUP_PRICE_V2: &str = "https://api.jup.ag/price/v2?ids=";

/// Pyth API URL
pub const PYTH_API: &str = "https://hermes.pyth.network/v2";
