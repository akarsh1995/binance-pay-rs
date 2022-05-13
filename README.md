# binance-pay-rs

Unofficial Rust Library for the [Binance Pay API](https://developers.binance.com/docs/binance-pay/introduction)


> :warning: The APIs are expected to work fine, still, **You might encounter bugs. Please use at your own risk.**   
The current version implements all the API endpoints as documented in the Binance pay developer documentation.

## Quickstart

Make sure the following env variables are set:
  - `BINANCE_PAY_API_KEY`
  - `BINANCE_PAY_API_SECRET`
  

In your `Cargo.toml` file
```toml
[dependencies]
binance-pay-rs = "^0"
tokio = { version = "1.18.0", features = ["rt-multi-thread", "macros"] }
```

In your `main.rs` file

```rust
use bpay::api::order::create::{
    Currency, Env, Goods, GoodsCategory, GoodsType, Request as OrderRequest, TerminalType,
};
use bpay::api::Binance;
use bpay::client::Client;
use bpay::errors::Result;
use bpay::utils::create_nonce;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
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
    let create_order_result = order.post(&client).await?;
    println!(
        "This url can be sent across to complete the payment procedure: {}",
        create_order_result.universal_url
    );
    Ok(())
}
```

### To run an example: 
```sh
cargo run --example notification_axum_server
```

### Contributing:


Simply create a pull request. Properly documented code and tests.  

To run the tests:

```sh
cargo t
```
