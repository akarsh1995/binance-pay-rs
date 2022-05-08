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

```rs
use tokio;
use bpay::create_order::{Currency, Env, Goods, GoodsType, TerminalType};

#[tokio::main]
async fn main() {
    let client = Client::from_env();

    let order = Order {
        env: Env {
            terminal_type: TerminalType::Web,
        },
        merchant_trade_no: crate::utils::create_nonce(10),
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

    let create_order_result = order.post(client).await.unwrap();
    println!("{:?}", create_order_result);
}
```
# Roadmap

- [ ] v0.2.0
    - [ ] Add documentation
    - [ ] Fetch certificate
    - [ ] Webhook notification signature verification
    - [ ] Order webhook notification
    - [ ] Query Order 
    - [ ] Close Order 
    - [ ] Refund Order
    - [ ] Refund Order Notification 
    - [ ] Wallet balance query 

- [ ] v0.3.0
    - [ ] Transfer Funds 
    - [ ] Query Transfer Result
    - [ ] Create Sub Merchant
    - [ ] Query Refund Order
    - [ ] Batch Payout
    - [ ] Payout Notification
    - [ ] Payout Query 
