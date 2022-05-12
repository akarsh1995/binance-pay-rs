//! API used to query wallet balance.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WalletType {
    FundingWallet,
    SpotWallet,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// Binance wallet to query, currently supported enum values.
    pub wallet: WalletType,

    /// Valid currency, must be in uppercase, e.g, "BUSD".
    pub currency: String,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, derive(Serialize))]
#[serde(rename_all = "camelCase")]
pub struct Response {
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

    use crate::c2b::tests::test_request_serialize_deserialize;

    test_request_serialize_deserialize!(
        (
            test_wallet_balance_serialize,
            r#"{"wallet":"SPOT_WALLET","currency":"BUSD"}"#,
            Request {
                wallet: WalletType::SpotWallet,
                currency: "BUSD".to_string(),
            }
        ),
        (
            test_wallet_balance_deserialize,
            r#"
            {
                "balance": 990000.00000000,
                "asset": "BUSD",
                "fiat": "USD",
                "availableFiatValuation": 989991.90516600,
                "availableBtcValuation": 22.98780000
            }
            "#,
            Response {
                balance: 990000.0,
                asset: "BUSD".to_string(),
                fiat: "USD".to_string(),
                available_fiat_valuation: 989991.90516600,
                available_btc_valuation: 22.9878,
            }
        )
    );
}
