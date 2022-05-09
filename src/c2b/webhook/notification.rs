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
}
# }
```
*/

pub mod order;

use serde::Deserialize;
use serde_json;

use crate::errors::Error;
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BizType {
    Pay,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BizStatus {
    PaySuccess,
    PayClosed,
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
}

impl TryFrom<NotificationRequestParams> for Notification {
    type Error = crate::errors::Error;

    fn try_from(params: NotificationRequestParams) -> crate::errors::Result<Self> {
        match params.biz_type {
            BizType::Pay => Ok(Self::Order(serde_json::from_str(&params.data)?)),
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
        }
    }
}
