//! API used to query wallet balance.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WalletType {
    FundingWallet,
    SpotWallet,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalance {
    /// Binance wallet to query, currently supported enum values.
    pub wallet: WalletType,

    /// Valid currency, must be in uppercase, e.g, "BUSD".
    pub currency: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalanceResult {
    /// Current balance free to use
    pub balance: f64,

    /// Asset/currency name
    pub asset: String,

    /// Fiat name, for fiat valuation
    pub fiat: String,

    /// Available fiat valuation
    pub available_fiat_valuation: f64,

    /// Available BTC valuation
    pub available_btc_valuation: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_bal_request_serialization() {
        let expected_request = r#"{"wallet":"SPOT_WALLET","currency":"BUSD"}"#;
        let wb_req = WalletBalance {
            wallet: WalletType::SpotWallet,
            currency: "BUSD".into(),
        };
        assert_eq!(serde_json::to_string(&wb_req).unwrap(), expected_request);
    }
}
