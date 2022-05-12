use bpay::api::close_order::CloseOrder;
use bpay::api::order::create::{
    Currency, Env, Goods, GoodsCategory, GoodsType, Request, Response, TerminalType,
};
use bpay::api::Binance;
use bpay::c2b::close_order::CloseOrderResult;
use bpay::client::Client;
use tokio;

async fn create_dummy_order(
    merchant_trade_no: &str,
    client: &Client,
) -> bpay::errors::Result<Response> {
    let order = Request {
        env: Env {
            terminal_type: TerminalType::Web,
        },
        merchant_trade_no: merchant_trade_no.to_string(),
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

    Ok(order.post(client).await?)
}

#[tokio::main]
async fn main() -> bpay::errors::Result<()> {
    let merchant_trade_no = "axLb12e3";
    let client = Client::from_env();
    let _dummy_order = create_dummy_order(merchant_trade_no, &client).await?;
    // do something with dummy_order
    // or
    // close the order
    let close_order_request = CloseOrder::new(None, Some(merchant_trade_no.to_string()));

    let close_order_result = close_order_request.post(&client).await?;
    match close_order_result {
        CloseOrderResult::Success => println!("Order closed successfully"),
        CloseOrderResult::Failure => println!("Order could not be closed"),
    }
    Ok(())
}
