//! Close order API used for merchant/partner to close order
//! without any prior payment activities triggered by user.
//! The successful close result will be notified asynchronously
//! through Order Notification Webhook with bizStatus = "PAY_CLOSED"

use serde::{Deserialize, Serialize};

/// Either of the prepay id or the merchant trade no must be present.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CloseOrder {
    // maximum length 32,letter or digit, no other symbol allowed, can not be empty if prepayId is empty
    prepay_id: Option<String>,

    // maximum length 19,letter or digit, no other symbol allowed, can not be empty if merchantTradeNo is empty
    merchant_trade_no: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CloseOrderResult(pub bool);

impl CloseOrder {
    pub fn new(prepay_id: Option<String>, merchant_trade_no: Option<String>) -> Self {
        assert!(prepay_id.is_some() || merchant_trade_no.is_some());
        Self {
            prepay_id,
            merchant_trade_no,
        }
    }
}
