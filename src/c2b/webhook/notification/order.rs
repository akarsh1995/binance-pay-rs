//! Order notification json deserialization format.

pub use crate::c2b::create_order::{Currency, TerminalType};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderNotification {
    ///	letter or digit, no other symbol allowed	The order id, Unique identifier for the request
    pub merchant_trade_no: String,

    ///	maximum length 16	product type
    pub product_type: String,

    ///	maximum length 256	product name
    pub product_name: String,

    ///	"WEB", "APP", "WAP", "MINI_PROGRAM", "PAYMENT_LINK", "OTHERS"	operate entrance
    pub trade_type: TerminalType,

    ///	order amount
    pub total_fee: f64,

    ///	String order currency
    pub currency: Currency,

    ///	Consumer unique id
    pub open_user_id: Option<String>,
}
