/*!
Webhook notification payload parsing helpers.
```
# use bpay::c2b::webhook::notification::{Notification, order::Currency};
# use bpay::c2b::webhook::notification::order::BizStatus;
# fn main() {
let body = r#"{
    "bizType": "PAY",
    "data": "{\"merchantTradeNo\":\"9825382937292\",\"totalFee\":0.88000000,\"transactTime\":1619508939664,\"currency\":\"USDT\",\"openUserId\":\"1211HS10K81f4273ac031\",\"productType\":\"Food\",\"productName\":\"Ice Cream\",\"tradeType\":\"WEB\",\"transactionId\":\"M_R_282737362839373\"}",
    "bizId": 29383937493038367292,
    "bizStatus": "PAY_SUCCESS"
}"#;

let notification = Notification::try_from(body).unwrap();
match notification {
    Notification::Order {
        biz_id,
        biz_status,
        order_detail,
    } => {
        assert_eq!(biz_id, 29383937493038367292);
        assert_eq!(biz_status, BizStatus::PaySuccess);
        # assert_eq!(order_detail.merchant_trade_no, "9825382937292");
        # assert_eq!(order_detail.total_fee, 0.88);
        # assert_eq!(order_detail.currency, Currency::USDT);
    }
    _ => panic!("Unexpected notification type"),
}
# }
```
*/

pub mod order;
pub mod refund;

use serde::Deserialize;
use serde_json::{self, Value};

use crate::errors::Error;
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BizType {
    Pay,
    PayRefund,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NotificationRequestParams {
    ///	string	Y	-	"PAY"
    pub biz_type: BizType,
    ///	string	Y	-	Prepay order id
    pub biz_id: u128,
    ///	string	Y	-	"PAY_SUCCESS"",PAY_CLOSED"
    pub biz_status: Value,
    ///	string	Y	-	JSON string, data details refer to
    pub data: String,
}

impl TryFrom<&str> for NotificationRequestParams {
    type Error = Error;

    fn try_from(value: &str) -> crate::errors::Result<Self> {
        Ok(serde_json::from_str::<NotificationRequestParams>(value)?)
    }
}

#[derive(Deserialize, Debug)]
pub enum Notification {
    Order {
        biz_id: u128,
        biz_status: order::BizStatus,
        order_detail: order::OrderNotification,
    },
    Refund {
        biz_id: u128,
        biz_status: refund::BizStatus,
        refund_detail: refund::Refund,
    },
}

impl TryFrom<NotificationRequestParams> for Notification {
    type Error = crate::errors::Error;

    fn try_from(params: NotificationRequestParams) -> crate::errors::Result<Self> {
        match params.biz_type {
            BizType::Pay => Ok(Notification::Order {
                biz_id: params.biz_id,
                biz_status: serde_json::from_value::<order::BizStatus>(params.biz_status)?,
                order_detail: serde_json::from_str::<order::OrderNotification>(&params.data)?,
            }),
            BizType::PayRefund => Ok(Notification::Refund {
                biz_id: params.biz_id,
                biz_status: serde_json::from_value::<refund::BizStatus>(params.biz_status)?,
                refund_detail: serde_json::from_str::<refund::Refund>(&params.data)?,
            }),
        }
    }
}

impl TryFrom<&str> for Notification {
    type Error = crate::errors::Error;

    fn try_from(value: &str) -> crate::errors::Result<Self> {
        Notification::try_from(NotificationRequestParams::try_from(value)?)
    }
}

#[cfg(test)]
mod tests {
    use order::Currency;

    use super::*;

    #[test]
    fn test_notification_from_str() {
        let body = r#"{
            "bizType": "PAY",
            "data": "{\"merchantTradeNo\":\"9825382937292\",\"totalFee\":0.88000000,\"transactTime\":1619508939664,\"currency\":\"USDT\",\"openUserId\":\"1211HS10K81f4273ac031\",\"productType\":\"Food\",\"productName\":\"Ice Cream\",\"tradeType\":\"WEB\",\"transactionId\":\"M_R_282737362839373\"}",
            "bizId": 29383937493038367292,
            "bizStatus": "PAY_SUCCESS"
          }"#;
        let notification = Notification::try_from(body).unwrap();
        match notification {
            Notification::Order {
                biz_id,
                biz_status,
                order_detail: details,
            } => {
                assert_eq!(biz_id, 29383937493038367292);
                assert_eq!(biz_status, order::BizStatus::PaySuccess);
                assert_eq!(details.merchant_trade_no, "9825382937292");
                assert_eq!(details.total_fee, 0.88);
                assert_eq!(details.currency, Currency::USDT);
            }
            _ => panic!("Unexpected notification type"),
        }
    }

    #[test]
    fn test_refund_notification_parsing() {
        let body = r#"
        {
            "bizType": "PAY_REFUND",
            "data": "{\"merchantTradeNo\":\"6177e6ae81ce6f001b4a6233\",\"totalFee\":0.01,\"transactTime\":1635248421335,\"refundInfo\":{\"orderAmount\":\"0.01000000\",\"duplicateRequest\":\"N\",\"payerOpenId\":\"9aa0a8bb21cf5fbf049aad7db35dc3d3\",\"prepayId\":\"123289163323899904\",\"refundRequestId\":\"68711039982968853\",\"refundedAmount\":\"0.01000000\",\"remainingAttempts\":9,\"refundAmount\":\"0.01000000\"},\"currency\":\"BUSD\",\"commission\":0,\"openUserId\":\"b5ec36baaa5ab9a5cfb1c29c2057bd81\",\"productType\":\"LIVE_STREAM\",\"productName\":\"LIVE_STREAM\",\"tradeType\":\"APP\"}",
            "bizId": 123289163323899904,
            "bizStatus": "REFUND_SUCCESS"
        }
        "#;
        let notification = Notification::try_from(body).unwrap();
        match notification {
            Notification::Refund {
                biz_id,
                biz_status,
                refund_detail: details,
            } => {
                assert_eq!(biz_id, 123289163323899904);
                assert_eq!(biz_status, refund::BizStatus::RefundSuccess);
                assert_eq!(details.merchant_trade_no, "6177e6ae81ce6f001b4a6233");
                assert_eq!(details.refund_info.order_amount, "0.01000000");
            }
            _ => panic!("Unexpected notification type"),
        }
    }
}
