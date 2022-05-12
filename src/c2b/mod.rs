//! Contains all the possible Serializable and Deserializable
//! request and response structs inside respective modules.
pub mod order;
pub mod payout;
pub mod refund;
pub mod sub_merchant;
pub mod transfer;
pub mod wallet_balance;
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
