# binance-pay-rs

Unofficial Rust Library for the [Binance Pay API](https://developers.binance.com/docs/binance-pay/introduction)


# Quickstart

Make sure the following env variables are set:
  - `BINANCE_PAY_API_KEY`
  - `BINANCE_PAY_API_SECRET`
  

In your `Cargo.toml` file
```toml
[dependencies]
binance-pay-rs = "^0"
tokio = { version = "1.18.0", features = ["rt-multi-thread", "macros"] }
```

## Example 

```rust
use bpay::api::create_order::{
   Currency, Env, Goods, GoodsCategory, GoodsType, Order, TerminalType,
};
use bpay::api::Binance;
use bpay::client::Client;
use bpay::utils::create_nonce;
use tokio;

#[tokio::main]
async fn main() {

   let order = Order {
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

   let mut client = Client::from_env();
   let create_order_result = order.post(&client).await.unwrap();
   match create_order_result.terminal_type {
       TerminalType::Web => assert!(true),
       _ => assert!(false),
   }
}
```

### To run an example: 
```sh
cargo run --example notification_axum_server
```

# Roadmap

- [x] v0.2.2
    - [x] more doc codeblocks
        - [x] Verification codeblock `src/c2b/webhook/verification` 
        - [x] Notification codeblock `src/c2b/webhook/notification` 

- [ ] v0.3.0
    - [ ] Close Order enum variants Success or Fail instead of bool
    - [ ] Transfer Funds 
    - [ ] Query Transfer Result
    - [ ] Create Sub Merchant
    - [ ] Query Refund Order
    - [ ] Batch Payout
    - [ ] Payout Notification
    - [ ] Payout Query 
