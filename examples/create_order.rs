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
