//! This library provides access to Binance pay APIs.
//! Refer: [Binance Pay Documentation](https://developers.binance.com/docs/binance-pay/introduction).
//!
//! # Quickstart
//!
//! Make sure the following env variables are set:
//!   - `BINANCE_PAY_API_KEY`
//!   - `BINANCE_PAY_API_SECRET`
//!
//! In your `Cargo.toml`, add the following:
//!
//! ```toml
//! binance-pay-rs = "^0"
//! tokio = { version = "1.18.0", features = ["rt-multi-thread", "macros"] }
//! ```
//!
//! ## Example
//! ```
//! use bpay::api::create_order::{
//!     Currency, Env, Goods, GoodsCategory, GoodsType, Order, TerminalType,
//! };
//! use bpay::api::Binance;
//! use bpay::client::Client;
//! use bpay::utils::create_nonce;
//! use tokio;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::from_env();
//!
//!     let order = Order {
//!         env: Env {
//!             terminal_type: TerminalType::Web,
//!         },
//!         merchant_trade_no: create_nonce(10),
//!         order_amount: 10.0,
//!         currency: Currency::USDT,
//!         goods: Goods {
//!             goods_type: GoodsType::VirtualGoods,
//!             goods_category: GoodsCategory::Electronics,
//!             reference_goods_id: "sku1234".into(),
//!             goods_name: "Laptop".into(),
//!             goods_detail: None,
//!         },
//!     };
//!
//!     let create_order_result = order.post(client).await.unwrap();
//!     println!("{:?}", create_order_result);
//! }
//! ```

pub mod api;
pub mod c2b;
pub mod client;
pub mod errors;
pub mod utils;
