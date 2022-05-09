//! Refund order API used for Merchant/Partner to refund for a successful payment.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RefundOrder {
    /// The unique ID assigned by the merchant to identify a refund request.The value must be same for one refund request.
    pub refund_request_id: String,

    /// The unique ID assigned by Binance for the original order to be refunded.
    pub prepay_id: String,

    /// You can perform multiple partial refunds, but their sum should not exceed the order amount.
    pub refund_amount: f64,

    /// Reason of the refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum RefundDuplicateStatus {
    #[serde(rename = "Y")]
    Yes,

    #[serde(rename = "N")]
    No,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RefundResult {
    ///  The unique ID assigned by the merchant to identify a refund request.
    pub refund_request_id: String,

    ///  The unique ID assigned by Binance for the original order to be refunded.
    pub prepay_id: String,

    ///  The total amount of prepay order.
    pub order_amount: String,

    ///  The total refunded amount included this refund request.
    pub refunded_amount: String,

    ///  The refund amount of this refund request.
    pub refund_amount: String,

    ///  The remaining attempts of this original order. If this value becomes 1, then your next refund request amount will be ignored. We will refund all the remaing amount of this original order.
    pub remaining_attempts: u8,

    ///  The payer open id of this refund which is the merchant open id.
    pub payer_open_id: String,

    ///  The flag to mark this request refundRequestId is duplicate or not. It will be 'Y' or 'N'
    pub duplicate_request: RefundDuplicateStatus,
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;

    #[test]
    fn test_refund_order_request_serialization() {
        let expected_request = r#"{"refundRequestId":"68711039982968832","prepayId":"383729303729303","refundAmount":25.00,"refundReason":""}"#;
        let refund_order = RefundOrder {
            refund_request_id: "68711039982968832".into(),
            prepay_id: "383729303729303".into(),
            refund_amount: 25.00,
            refund_reason: Some("".into()),
        };
        assert_eq!(
            serde_json::to_value(&refund_order).unwrap(),
            serde_json::from_str::<Value>(expected_request).unwrap()
        );
    }
}
