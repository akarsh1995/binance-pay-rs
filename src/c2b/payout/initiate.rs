use serde::{Deserialize, Serialize};

pub use crate::c2b::balance_query::WalletType as TransferMethod;

#[derive(Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BizScene {
    /// The default value
    DirectTransfer,

    /// Gift or rewards payout
    CryptoRewards,

    /// Settlement or commission fee
    Settlement,

    /// Reimburse employees
    Reimbursement,

    /// Tayment to partners/users
    MerchantPayment,

    Others,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReceiveType {
    PayId,
    BinanceId,
    Email,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferDetailReq {
    /// The unique ID assigned by the merchant to identify a detail transfer.
    merchant_send_id: String,

    /// Receiver's ID type  support payout to non-binance users
    receive_type: ReceiveType,

    /// If it is a non-binance user email address, he or she needs to register a binance account to receive the payout.  
    /// The amount will be automatically refunded to your wallet if receivers haven't completed the registration with 72 hours.
    receiver: String,

    /// The transfer value cannot be less than 2 USD Transfer amount.
    transfer_amount: f64,

    transfer_method: TransferMethod,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum length 128	Remark
    remark: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// The unique ID assigned by the merchant to identify a payout request.
    request_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Describe the business scene of this payout.
    biz_scene: Option<BizScene>,

    /// The name of the batch payout.
    batch_name: String,

    /// Crypto token only, fiat NOT supported. All characters must be in uppercase	All the transfers under this batch must use the same currency.
    currency: String,

    /// It must be equal to the sum of all the detail transfers.
    total_amount: f64,

    /// The total number of transfers. It must be equal to the detail transfer count.
    total_number: u8,

    /// Detail transfer list
    transfer_detail_list: Vec<TransferDetailReq>,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, derive(Serialize))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Accepted,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, derive(Serialize))]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub request_id: String,
    pub status: Status,
}

#[cfg(test)]
mod tests {
    use crate::c2b::tests::test_request_serialize_deserialize;
    test_request_serialize_deserialize!(
        (
            test_batch_payout_serialize,
            r#"
        {
            "requestId": "samplerequest1234",
            "batchName": "sample batch",
            "currency": "BUSD",
            "totalAmount": 200.4,
            "totalNumber": 2,
            "bizScene": "SETTLEMENT",
            "transferDetailList": [
              {
                "merchantSendId": "22231313131",
                "transferAmount": 110.3,
                "receiveType": "PAY_ID",
                "transferMethod": "SPOT_WALLET",
                "receiver": "354205155",
                "remark": "test1"
              },
              {
                "merchantSendId": "21231313132",
                "transferAmount": 90.1,
                "receiveType": "PAY_ID",
                "transferMethod": "SPOT_WALLET",
                "receiver": "354205153",
                "remark": "test2"
              }
            ]
          } 
        "#,
            Request {
                request_id: "samplerequest1234".to_string(),
                batch_name: "sample batch".to_string(),
                currency: "BUSD".to_string(),
                total_amount: 200.4,
                total_number: 2,
                biz_scene: Some(BizScene::Settlement),
                transfer_detail_list: vec![
                    TransferDetailReq {
                        merchant_send_id: "22231313131".to_string(),
                        transfer_amount: 110.3,
                        receive_type: ReceiveType::PayId,
                        transfer_method: TransferMethod::SpotWallet,
                        receiver: "354205155".to_string(),
                        remark: Some("test1".to_string()),
                    },
                    TransferDetailReq {
                        merchant_send_id: "21231313132".to_string(),
                        transfer_amount: 90.1,
                        receive_type: ReceiveType::PayId,
                        transfer_method: TransferMethod::SpotWallet,
                        receiver: "354205153".to_string(),
                        remark: Some("test2".to_string()),
                    },
                ],
            }
        ),
        (
            test_batch_payout_deserialize,
            r#"{"requestId":"samplerequest1234","status":"ACCEPTED"}"#,
            Response {
                request_id: "samplerequest1234".to_string(),
                status: Status::Accepted,
            }
        )
    );
}
