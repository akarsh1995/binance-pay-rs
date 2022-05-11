//! Refund order API used for Merchant/Partner to refund for a successful payment.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueryRefundRequest {
    /// The unique ID assigned by the merchant to identify a refund request.
    pub refund_request_id: String,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, derive(Serialize))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefundStatus {
    RefundSuccess,
    RefundFail,
    RefundPending,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, derive(Serialize))]
#[serde(rename_all = "camelCase")]
pub struct QueryRefundResult {
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

    /// The status of this refund. Example: REFUND_SUCCESS,REFUND_FAIL,REFUND_PENDING
    pub refund_status: RefundStatus,
}

#[cfg(test)]
mod tests {
    use crate::c2b::tests::test_request_serialize_deserialize;

    test_request_serialize_deserialize!(
        (
            test_query_refund_request_serialize,
            r#"{  "refundRequestId": "68711039982968832"}"#,
            QueryRefundRequest {
                refund_request_id: "68711039982968832".to_string(),
            }
        ),
        (
            test_query_refund_result_deserialize,
            r#"{"refundRequestId":"68711039982968832","prepayId":"383729303729303","orderAmount":"100.11","refundedAmount":"10.88","refundAmount":"5.00","remainingAttempts":8,"payerOpenId":"dde730c2e0ea1f1780cf26343b98fd3b","refundStatus":"REFUND_SUCCESS"}"#,
            QueryRefundResult {
                refund_request_id: "68711039982968832".to_string(),
                prepay_id: "383729303729303".to_string(),
                order_amount: "100.11".to_string(),
                refunded_amount: "10.88".to_string(),
                refund_amount: "5.00".to_string(),
                remaining_attempts: 8,
                payer_open_id: "dde730c2e0ea1f1780cf26343b98fd3b".to_string(),
                refund_status: RefundStatus::RefundSuccess,
            }
        )
    );
}
