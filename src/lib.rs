/*!  This library provides access to Binance pay APIs.
Refer: [Binance Pay Documentation](https://developers.binance.com/docs/binance-pay/introduction).

# Quickstart

Make sure the following env variables are set:
  - `BINANCE_PAY_API_KEY`
  - `BINANCE_PAY_API_SECRET`

In your `Cargo.toml`, add the following:

```toml
binance-pay-rs = "^0"
tokio = { version = "1.18.0", features = ["rt-multi-thread", "macros"] }
```

 ## Example
 ```
use bpay::api::order::create::{
    Currency, Env, Goods, GoodsCategory, GoodsType, Request as OrderRequest, TerminalType,
};
use bpay::client::Client;
use bpay::utils::create_nonce;
# #[cfg(test)]
# use mockito;
# use mockito::mock;
use tokio;

#[tokio::main]
async fn main() {

    let order = OrderRequest {
        env: Env {
            terminal_type: TerminalType::Web,
        },
        merchant_trade_no: create_nonce(10),
        order_amount: 10.0,
        currency: Currency::USDT,
        goods: Goods {
            goods_type: GoodsType::VirtualGoods,
            goods_category: GoodsCategory::Electronics,
            reference_goods_id: "sku1234".into(),
            goods_name: "Laptop".into(),
            goods_detail: None,
        },
    };

    let client = Client::from_env();
    # let url = &mockito::server_url();
    # let response = r#"
    # {
    #     "status": "SUCCESS",
    #     "code": "000000",
    #     "data": {
    #       "prepayId": "29383937493038367292",
    #       "terminalType": "WEB",
    #       "expireTime": 121123232223,
    #       "qrcodeLink": "https://qrservice.dev.com/en/qr/dplkb005181944f84b84aba2430e1177012b.jpg",
    #       "qrContent": "https://qrservice.dev.com/en/qr/dplk12121112b",
    #       "checkoutUrl": "https://pay.binance.com/checkout/dplk12121112b",
    #       "deeplink": "bnc://app.binance.com/payment/secpay/xxxxxx",
    #       "universalUrl": "https://app.binance.com/payment/secpay?_dp=xxx=&linkToken=xxx"
    #     },
    #     "errorMessage": ""
    # }"#;
    # let mut client = Client::new(Some("".into()), Some("".into()), url.to_string());
    # let _m = mock("POST", "/binancepay/openapi/v2/order")
    #     .with_status(200)
    #     .with_header("content-type", "application/json")
    #     .with_body(response)
    #     .create();
    let create_order_result = order.create(&client).await.unwrap();
    println!(
        "This url can be sent across to complete the payment procedure: {}",
        create_order_result.universal_url
    );
}
 ```
*/
pub mod api;
pub mod c2b;
pub mod client;
pub mod errors;
pub mod utils;
