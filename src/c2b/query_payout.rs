//! Payout query API used for Merchant/Partner to query transfer status.

use serde::{Deserialize, Serialize};

pub use crate::c2b::payout::initiate::ReceiveType;
pub use crate::c2b::payout::initiate::TransferMethod;

#[derive(Serialize, Debug)]
pub enum DetailStatus {
    /// Return all transfer details, default value.  
    All,
    /// Only return transfers in processing.  
    Processing,
    /// Only return successful transfers.  
    Success,
    /// Only return failed transfers.
    Fail,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PayoutQueryRequest {
    /// The unique ID assigned by the merchant to identify a payout request.
    request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail_status: Option<DetailStatus>,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[cfg_attr(test, derive(Serialize))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BatchStatus {
    Accepted,
    Processing,
    Success,
    PartSuccess,
    Failed,
    Canceled,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, derive(Serialize))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Success,
    Fail,
    Processing,
    AwaitingReceipt,
    Refunded,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, derive(Serialize))]
#[serde(rename_all = "camelCase")]
pub struct TransferDetailResult {
    /// The passed-in request ID.
    pub order_id: u64,

    /// The passed-in merchantSendId.
    pub merchant_send_id: String,

    /// Payer's payment account ID.
    pub payer_id: u64,

    ///	Amount transferred.
    pub amount: String,

    ///	Enum string
    pub receive_type: ReceiveType,

    ///	Receiver ID from the request.
    pub receiver: String,

    ///	Receiver's payment account ID.
    pub payee_id: u64,

    ///	SPOT_WALLET FUNDING_WALLET
    pub transfer_method: TransferMethod,

    ///	SUCCESS FAIL PROCESSING AWAITING_RECEIPT REFUNDED
    pub status: Status,

    ///	Maximum length 128
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, derive(Serialize))]
#[serde(rename_all = "camelCase")]
pub struct TransferQueryResult {
    /// The passed-in request ID
    pub request_id: String,

    pub batch_status: BatchStatus,

    pub merchant_id: u64,

    /// The request currency.
    pub currency: String,

    /// Total transfer amount in this batch.
    /// Ambiguous: sample result response has type string but type mentioned is decimal.
    pub total_amount: f64,

    /// Total number of transfers in the batch.
    pub total_number: u16,

    /// Detail transfer result.
    pub transfer_detail_list: Vec<TransferDetailResult>,
}

#[cfg(test)]
mod tests {
    use crate::c2b::tests::test_request_serialize_deserialize;

    test_request_serialize_deserialize!(
        (
            test_payout_query_request_serialize,
            "{\"requestId\":\"payouttransfer19998\"}",
            PayoutQueryRequest {
                request_id: "payouttransfer19998".to_string(),
                detail_status: None,
            }
        ),
        (
            test_payout_query_result_deserialize,
            r#"
        {
            "requestId": "payoutqueryrequest1232455",
            "batchStatus": "ACCEPTED",
            "merchantId": 354195960,
            "currency": "USDT",
            "totalAmount": 0.00000002,
            "totalNumber": 2,
            "transferDetailList": [
              {
                "orderId": 118073402258677760,
                "merchantSendId": "21231313131",
                "payerId": 354195960,
                "amount": "0.00000001",
                "receiveType": "PAY_ID",
                "receiver": "354205155",
                "payeeId": 354205155,
                "transferMethod": "SPOT_WALLET",
                "status": "PROCESSING",
                "remark": "test1"
              },
              {
                "orderId": 118073402258677760,
                "merchantSendId": "21231313132",
                "payerId": 354195960,
                "amount": "0.00000001",
                "receiveType": "PAY_ID",
                "receiver": "354205155",
                "payeeId": 354205155,
                "transferMethod": "SPOT_WALLET",
                "status": "PROCESSING",
                "remark": "test2"
              }
            ]
          }
        "#,
            TransferQueryResult {
                request_id: "payoutqueryrequest1232455".to_string(),
                batch_status: BatchStatus::Accepted,
                merchant_id: 354195960,
                currency: "USDT".to_string(),
                total_amount: 0.00000002,
                total_number: 2,
                transfer_detail_list: vec![
                    TransferDetailResult {
                        order_id: 118073402258677760,
                        merchant_send_id: "21231313131".to_string(),
                        payer_id: 354195960,
                        amount: "0.00000001".to_string(),
                        receive_type: ReceiveType::PayId,
                        receiver: "354205155".to_string(),
                        payee_id: 354205155,
                        transfer_method: TransferMethod::SpotWallet,
                        status: Status::Processing,
                        remark: Some("test1".to_string()),
                    },
                    TransferDetailResult {
                        order_id: 118073402258677760,
                        merchant_send_id: "21231313132".to_string(),
                        payer_id: 354195960,
                        amount: "0.00000001".to_string(),
                        receive_type: ReceiveType::PayId,
                        receiver: "354205155".to_string(),
                        payee_id: 354205155,
                        transfer_method: TransferMethod::SpotWallet,
                        status: Status::Processing,
                        remark: Some("test2".to_string()),
                    },
                ],
            }
        )
    );
}
