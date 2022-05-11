//! Fund transfer API used for merchant/partner to initiate Fund transfer between wallets.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferType {
    ToMain,
    ToPay,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferFund {
    /// Represents the unique ID of each transfer request.Generated by the merchant
    request_id: String,

    /// Valid currency, must be in uppercase	transfer currency, e.g. "BUSD"
    currency: String,

    /// The transfer amount
    amount: String,

    /// The transfer direction specified by the merchant
    transfer_type: TransferType,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Success,
    Failure,
    Process,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransferFundResult {
    /// the value of Request property requestId
    pub tran_id: String,

    /// SUCCESS (indicating that the transfer is completely successful),
    /// FAILURE (indicating that the transfer has failed, it may be that the transferor has a problem with the transferee),
    /// PROCESS (the transfer is in progress)
    pub status: Status,
    pub currency: String,
    pub amount: String,
    pub transfer_type: TransferType,
}

macro_rules! test_request_serialize_deserialize {
    ($(($test_name: ident, $expected: literal, $p: expr),)*) => {
            #[cfg(test)]
            mod tests {
                use super::*;
                use serde_json::Value;
                $(
                    #[test]
                    fn $test_name() {
                        let expected_request_string = $expected;
                        let x = $p;
                        assert_eq!(
                            serde_json::to_value(&x).unwrap(),
                            serde_json::from_str::<Value>(expected_request_string).unwrap()
                        );
                    }
                )*
            }

    };
}

test_request_serialize_deserialize!(
    (
        test_transfer_fund_serialize,
        r#"{"requestId":"100002021071407140001","currency":"BNB","amount":"0.01","transferType":"TO_MAIN"}"#,
        TransferFund {
            request_id: "100002021071407140001".to_string(),
            currency: "BNB".to_string(),
            amount: "0.01".to_string(),
            transfer_type: TransferType::ToMain,
        }
    ),
    (
        test_transfer_fund_result_deserialize_to_pay,
        r#"{"tranId":"100002021071407140001","status":"SUCCESS","currency":"BNB","amount":"0.01","transferType":"TO_MAIN"}"#,
        TransferFundResult {
            tran_id: "100002021071407140001".to_string(),
            status: Status::Success,
            currency: "BNB".to_string(),
            amount: "0.01".to_string(),
            transfer_type: TransferType::ToMain,
        }
    ),
);
