//! Order request and response structure that's laid out in the API documentation.
//! [Create Order V2 Documentation](https://developers.binance.com/docs/binance-pay/api-order-create-v2)

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TerminalType {
    /// The client-side terminal type is a mobile application.
    App,

    /// The client-side terminal type is a website that is opened via a PC browser.
    Web,

    /// The client-side terminal type is an HTML page that is opened via a mobile browser.
    Wap,

    /// The terminal type of the merchant side is a mini program on the mobile phone.
    MiniProgram,

    /// other undefined type
    Others,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Env {
    pub terminal_type: TerminalType,
}

/// The type of the goods for the order
#[derive(Serialize, Debug)]
pub enum GoodsType {
    #[serde(rename = "01")]
    TangibleGoods,

    #[serde(rename = "02")]
    VirtualGoods,
}

#[derive(Serialize, Debug)]
pub enum GoodsCategory {
    #[serde(rename = "0000")]
    Electronics,

    #[serde(rename = "1000")]
    BooksMusicMovies,

    #[serde(rename = "2000")]
    HomeGardenTools,

    #[serde(rename = "3000")]
    ClothesShoesBags,

    #[serde(rename = "4000")]
    ToysKidsBaby,

    #[serde(rename = "5000")]
    AutomotiveAccessories,

    #[serde(rename = "6000")]
    GameRecharge,

    #[serde(rename = "7000")]
    EntertainamentCollection,

    #[serde(rename = "8000")]
    Jewelry,

    #[serde(rename = "9000")]
    DomesticService,

    #[serde(rename = "A000")]
    BeautyCare,

    #[serde(rename = "B000")]
    Pharmacy,

    #[serde(rename = "C000")]
    SportsOutdoors,

    #[serde(rename = "D000")]
    FoodGroceryHealth,

    #[serde(rename = "E000")]
    PetSupplies,

    #[serde(rename = "F000")]
    IndustryScience,

    #[serde(rename = "Z000")]
    Others,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Goods {
    pub goods_type: GoodsType,

    /// Goods category id.
    pub goods_category: GoodsCategory,

    /// The unique ID to identify the goods.
    pub reference_goods_id: String,

    /// Goods name limited to 256 characters.
    pub goods_name: String,

    /// Goods detail limited to 256 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_detail: Option<String>,
}

/// Order currency in upper case. only "BUSD","USDT","MBOX" can be accepted,
/// fiat NOT supported.
#[derive(Serialize, Debug)]
pub enum Currency {
    BUSD,
    USDT,
    MBOX,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    /// User's device environment information.
    pub env: Env,

    /// The order id, Unique identifier for the request letter or digit,
    /// no other symbol allowed, maximum length 32
    pub merchant_trade_no: String,

    /// Amount Range: `0.01` - `20000`
    pub order_amount: f32,

    pub currency: Currency,

    pub goods: Goods,
}

impl Default for Order {
    fn default() -> Self {
        Self {
            env: Env {
                terminal_type: TerminalType::Web,
            },
            merchant_trade_no: crate::utils::create_nonce(10),
            order_amount: 10.0,
            currency: Currency::USDT,
            goods: Goods {
                goods_type: GoodsType::VirtualGoods,
                goods_category: GoodsCategory::Electronics,
                reference_goods_id: "abcd".into(),
                goods_name: "XYZ".into(),
                goods_detail: None,
            },
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderResult {
    /// unique id generated by binance
    pub prepay_id: String,

    /// same as terminalType in request data
    pub terminal_type: TerminalType,

    /// expire time in milli seconds
    pub expire_time: u128,

    /// qr code img link
    pub qrcode_link: String,

    /// qr content decoded info
    pub qr_content: String,

    /// binance hosted checkout page url
    pub checkout_url: String,

    /// deeplink to open binance app to finish payment
    pub deeplink: String,

    /// Universal url to finish the payment.
    /// First tries with the mobile app, if not found, tries with the web browser
    pub universal_url: String,
}
