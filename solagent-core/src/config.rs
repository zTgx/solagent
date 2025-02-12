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

#![allow(dead_code)]

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub openai_api_key: Option<String>,
    pub jupiter_referral_account: Option<String>,
    pub jupiter_fee_bps: Option<u16>, // Assuming fee is represented as a percentage (0-10000)
    pub flash_privilege: Option<String>,
    pub flexlend_api_key: Option<String>,
    pub helius_api_key: Option<String>,
    pub cookie_api_key: Option<String>,
    pub birdeye_api_key: Option<String>,
}

#[derive(Default)]
pub struct ConfigBuilder {
    openai_api_key: Option<String>,
    jupiter_referral_account: Option<String>,
    jupiter_fee_bps: Option<u16>,
    flash_privilege: Option<String>,
    flexlend_api_key: Option<String>,
    helius_api_key: Option<String>,
    cookie_api_key: Option<String>,
    birdeye_api_key: Option<String>,
}

impl ConfigBuilder {
    pub fn openai_api_key(mut self, key: String) -> Self {
        self.openai_api_key = Some(key);
        self
    }

    pub fn jupiter_referral_account(mut self, account: String) -> Self {
        self.jupiter_referral_account = Some(account);
        self
    }

    pub fn jupiter_fee_bps(mut self, fee: u16) -> Self {
        self.jupiter_fee_bps = Some(fee);
        self
    }

    pub fn flash_privilege(mut self, privilege: String) -> Self {
        self.flash_privilege = Some(privilege);
        self
    }

    pub fn flexlend_api_key(mut self, key: String) -> Self {
        self.flexlend_api_key = Some(key);
        self
    }

    pub fn helius_api_key(mut self, key: String) -> Self {
        self.helius_api_key = Some(key);
        self
    }

    pub fn cookie_api_key(mut self, key: String) -> Self {
        self.cookie_api_key = Some(key);
        self
    }

    pub fn birdeye_api_key(mut self, key: String) -> Self {
        self.birdeye_api_key = Some(key);
        self
    }

    pub fn build(self) -> Config {
        Config {
            openai_api_key: self.openai_api_key,
            jupiter_referral_account: self.jupiter_referral_account,
            jupiter_fee_bps: self.jupiter_fee_bps,
            flash_privilege: self.flash_privilege,
            flexlend_api_key: self.flexlend_api_key,
            helius_api_key: self.helius_api_key,
            cookie_api_key: self.cookie_api_key,
            birdeye_api_key: self.birdeye_api_key,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder_default() {
        let config = ConfigBuilder::default().build();
        assert!(config.openai_api_key.is_none());
        assert!(config.jupiter_referral_account.is_none());
        assert_eq!(config.jupiter_fee_bps, None);
        assert!(config.flash_privilege.is_none());
        assert!(config.flexlend_api_key.is_none());
        assert!(config.helius_api_key.is_none());
        assert!(config.cookie_api_key.is_none());
        assert!(config.birdeye_api_key.is_none());
    }

    #[test]
    fn test_config_builder_with_values() {
        let config = ConfigBuilder::default()
            .openai_api_key("test_api_key".to_string())
            .jupiter_referral_account("test_referral_account".to_string())
            .jupiter_fee_bps(500)
            .flash_privilege("test_flash_privilege".to_string())
            .flexlend_api_key("test_flexlend_key".to_string())
            .helius_api_key("test_helius_key".to_string())
            .cookie_api_key("test_cookie_key".to_string())
            .birdeye_api_key("birdeye_api_key".to_string())
            .build();

        assert_eq!(config.openai_api_key, Some("test_api_key".to_string()));
        assert_eq!(config.jupiter_referral_account, Some("test_referral_account".to_string()));
        assert_eq!(config.jupiter_fee_bps, Some(500));
        assert_eq!(config.flash_privilege, Some("test_flash_privilege".to_string()));
        assert_eq!(config.flexlend_api_key, Some("test_flexlend_key".to_string()));
        assert_eq!(config.helius_api_key, Some("test_helius_key".to_string()));
        assert_eq!(config.cookie_api_key, Some("test_cookie_key".to_string()));
        assert_eq!(config.birdeye_api_key, Some("birdeye_api_key".to_string()));
    }
}
