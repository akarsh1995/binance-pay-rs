//! This library provides access to Binance pay APIs.
//! Refer: [Binance Pay Documentation](https://developers.binance.com/docs/binance-pay/introduction).
//!
//! ## Quickstart
//!
//! Make sure the following env variables are set:
//!   - `BINANCE_API_KEY`
//!   - `BINANCE_API_SECRET`
//! ```
//! use bpay::create_order::{Currency, Env, Goods, GoodsType, TerminalType};
//!
//! let client = Client::from_env();
//!
//! let order = Order {
//!     env: Env {
//!         terminal_type: TerminalType::Web,
//!     },
//!     merchant_trade_no: crate::utils::create_nonce(10),
//!     order_amount: 10.0,
//!     currency: Currency::USDT,
//!     goods: Goods {
//!         goods_type: GoodsType::VirtualGoods,
//!         goods_category: GoodsCategory::Electronics,
//!         reference_goods_id: "sku1234".into(),
//!         goods_name: "Laptop".into(),
//!         goods_detail: None,
//!     },
//! };
//!
//! let create_order_result = order.post(client).await.unwrap();
//! println!("{:?}", create_order_result);
//! ```

pub mod api;
mod c2b;
pub mod client;
pub mod errors;
pub mod utils;

use api::API;
pub use c2b::*;
use errors::Result;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
struct Response {
    status: String,

    code: String,

    data: Value,

    #[serde(rename = "errorMessage")]
    error_message: Option<String>,
}

#[async_trait::async_trait]
trait Binance<D>: Serialize + Sized
where
    D: DeserializeOwned,
{
    async fn post(&self, client: client::Client) -> Result<D> {
        let response = client
            .post_signed_s::<Response, Self>(self.get_api().into(), Some(self))
            .await?;
        Ok(serde_json::from_value(response.data)?)
    }

    fn get_api(&self) -> API;
}

impl Binance<c2b::create_order::CreateOrderResult> for c2b::create_order::Order {
    fn get_api(&self) -> API {
        API::CreateOrder
    }
}
