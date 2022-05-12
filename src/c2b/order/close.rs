//! Close order API used for merchant/partner to close order
//! without any prior payment activities triggered by user.
//! The successful close result will be notified asynchronously
//! through Order Notification Webhook with bizStatus = "PAY_CLOSED"

use std::fmt;

use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};

/// Either of the prepay id or the merchant trade no must be present.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    // maximum length 32,letter or digit, no other symbol allowed, can not be empty if prepayId is empty
    prepay_id: Option<String>,

    // maximum length 19,letter or digit, no other symbol allowed, can not be empty if merchantTradeNo is empty
    merchant_trade_no: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(Serialize))]
pub enum Response {
    Success,
    Failure,
}

impl<'de> Deserialize<'de> for Response {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_bool(ClosedStatusVisitor)
    }
}

struct ClosedStatusVisitor;

impl<'de> Visitor<'de> for ClosedStatusVisitor {
    type Value = Response;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a boolean value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Response, E>
    where
        E: de::Error,
    {
        match value {
            true => Ok(Response::Success),
            false => Ok(Response::Failure),
        }
    }
}

impl Request {
    pub fn new(prepay_id: Option<String>, merchant_trade_no: Option<String>) -> Self {
        assert!(prepay_id.is_some() || merchant_trade_no.is_some());
        Self {
            prepay_id,
            merchant_trade_no,
        }
    }
}

// #[cfg(test)]
// mod tests {
#[cfg(test)]
mod tests {
    use crate::c2b::tests::test_request_serialize_deserialize;
    test_request_serialize_deserialize!((
        test_serialize_close_order,
        r#"{"merchantTradeNo":"9825382937292","prepayId":null}"#,
        Request {
            prepay_id: None,
            merchant_trade_no: Some("9825382937292".into()),
        }
    ));

    #[test]
    fn test_deserialize_close_order_result() {
        let result_json_true = r#"true"#;
        let result_json_false = r#"false"#;
        assert_eq!(
            serde_json::from_str::<Response>(result_json_true).unwrap(),
            Response::Success
        );
        assert_eq!(
            serde_json::from_str::<Response>(result_json_false).unwrap(),
            Response::Failure
        );
    }
}
