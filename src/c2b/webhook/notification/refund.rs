pub use crate::c2b::refund::create::RefundResult as RefundInfo;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BizStatus {
    RefundSuccess,
    RefundRejected,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Refund {
    /// The order id, Unique identifier for the request
    pub merchant_trade_no: String,
    /// product type
    pub product_type: String,

    /// product name
    pub product_name: String,

    /// 	string	Y	"WEB", "APP", "WAP", "MINI_PROGRAM", "PAYMENT_LINK", "OTHERS"	operate entrance
    pub trade_type: String,

    /// order amount
    pub total_fee: f64,

    /// order currency
    pub currency: String,

    /// Consumer unique id
    pub open_user_id: String,

    /// Only merchant got approved by Binance Operation's approval will receive this payerInfo	payer information, refer to
    pub refund_info: RefundInfo,
}
