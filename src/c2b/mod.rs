//! Contains all the possible Serializable and Deserializable
//! request and response structs inside respective modules.
pub mod balance_query;
pub mod batch_payout;
pub mod close_order;
pub mod create_order;
pub mod query_order;
pub mod query_refund;
pub mod query_transfer;
pub mod refund_order;
pub mod transfer_fund;
pub mod webhook;

#[cfg(test)]
mod tests {
    macro_rules! test_request_serialize_deserialize {
        ($(($test_name: ident, $expected: literal, $p: expr)),+) => {
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
            )+


        };
    }
    pub(crate) use test_request_serialize_deserialize;
}
