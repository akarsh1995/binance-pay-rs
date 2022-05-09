/*!
Webhook notification payload parsing helpers.
```
# use bpay::c2b::webhook::notification::{Notification, order::Currency};
# fn main() {
let body = r#"{
    "bizType": "PAY",
    "data": "{\"merchantTradeNo\":\"9825382937292\",\"totalFee\":0.88000000,\"transactTime\":1619508939664,\"currency\":\"USDT\",\"openUserId\":\"1211HS10K81f4273ac031\",\"productType\":\"Food\",\"productName\":\"Ice Cream\",\"tradeType\":\"WEB\",\"transactionId\":\"M_R_282737362839373\"}",
    "bizId": 29383937493038367292,
    "bizStatus": "PAY_SUCCESS"
}"#;

let notification = Notification::try_from(body).unwrap();
match notification {
    Notification::Order(order) => {
        assert_eq!(order.merchant_trade_no, "9825382937292");
        assert_eq!(order.total_fee, 0.88);
        assert_eq!(order.currency, Currency::USDT);
    }
    _ => panic!("Unexpected notification type"),
}
# }
```
*/

pub mod order;
pub mod refund;

use serde::Deserialize;
use serde_json;

use crate::errors::Error;
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BizType {
    Pay,
    PayRefund,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BizStatus {
    PaySuccess,
    PayClosed,
    RefundSuccess,
    RefundRejected,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NotificationRequestParams {
    ///	string	Y	-	"PAY"
    pub biz_type: BizType,
    ///	string	Y	-	Prepay order id
    pub biz_id: u128,
    ///	string	Y	-	"PAY_SUCCESS"",PAY_CLOSED"
    pub biz_status: BizStatus,
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
    Order(order::OrderNotification),
    Refund(refund::Refund),
}

impl TryFrom<NotificationRequestParams> for Notification {
    type Error = crate::errors::Error;

    fn try_from(params: NotificationRequestParams) -> crate::errors::Result<Self> {
        match params.biz_type {
            BizType::Pay => Ok(Self::Order(serde_json::from_str(&params.data)?)),
            BizType::PayRefund => Ok(Self::Refund(serde_json::from_str(&params.data)?)),
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
            Notification::Order(order) => {
                assert_eq!(order.merchant_trade_no, "9825382937292");
                assert_eq!(order.total_fee, 0.88);
                assert_eq!(order.currency, Currency::USDT);
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
            Notification::Refund(refund) => {
                assert_eq!(refund.merchant_trade_no, "6177e6ae81ce6f001b4a6233");
                assert_eq!(refund.refund_info.order_amount, "0.01000000");
            }
            _ => panic!("Unexpected notification type"),
        }
    }
}
