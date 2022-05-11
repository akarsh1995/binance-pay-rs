//! Use Binance [`client::Client`] in conjunction with the Request Response structs.
//! - [`create_order::Order`] and [`create_order::CreateOrderResult`]

use self::{
    balance_query::{WalletBalance, WalletBalanceResult},
    close_order::{CloseOrder, CloseOrderResult},
    create_order::{CreateOrderResult, Order},
    query_order::{QueryOrder, QueryOrderResult},
    refund_order::{RefundDuplicateStatus, RefundOrder, RefundResult},
    webhook::certificate::{Certificate, CertificateResult},
};
use crate::c2b::webhook::verification::Verifier;
pub use crate::c2b::*;
use crate::client;
use crate::client::Client;
use crate::errors::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

pub enum API {
    CreateOrder,
    QueryCertificate,
    QueryOrder,
    CloseOrder,
    RefundOrder,
    BalanceQuery,
}

impl From<API> for String {
    fn from(item: API) -> Self {
        String::from(match item {
            API::CreateOrder => "/binancepay/openapi/v2/order",
            API::QueryCertificate => "/binancepay/openapi/certificates",
            API::QueryOrder => "/binancepay/openapi/order/query",
            API::CloseOrder => "/binancepay/openapi/order/close",
            API::RefundOrder => "/binancepay/openapi/order/refund",
            API::BalanceQuery => "/binancepay/openapi/balance",
        })
    }
}

/// Response format from the Binance Pay API.
#[derive(Deserialize)]
struct Response {
    status: String,

    code: String,

    data: Value,

    #[serde(rename = "errorMessage")]
    error_message: Option<String>,
}

#[async_trait::async_trait]
pub trait Binance<D>: Serialize + Sized
where
    D: DeserializeOwned,
{
    async fn post(&self, client: &client::Client) -> Result<D> {
        let response = client
            .post_signed_s::<Response, Self>(self.get_api().into(), Some(self))
            .await?;
        Ok(serde_json::from_value(response.data)?)
    }

    fn get_api(&self) -> API;
}

#[macro_export]
macro_rules! impl_binance {
    ($(($x: ty, $y: ty, $z: ident)),+) => {
            $(
                impl Binance<$x> for $y {
                    fn get_api(&self) -> API {
                        API::$z
                    }
                }
            )+
    };
}

impl_binance!(
    (CreateOrderResult, Order, CreateOrder),
    (Vec<CertificateResult>, Certificate, QueryCertificate),
    (QueryOrderResult, QueryOrder, QueryOrder),
    (CloseOrderResult, CloseOrder, CloseOrder),
    (RefundResult, RefundOrder, RefundOrder),
    (WalletBalanceResult, WalletBalance, BalanceQuery)
);

/// Get certificate out of the received response array.
pub async fn get_certificate(client: &Client) -> Result<CertificateResult> {
    let mut certs_arr = Certificate.post(client).await?;
    Ok(certs_arr.pop().expect("No certificates found"))
}

/// Get [`Verifier`] directly from the api.
/// ```rust,no_run
/// # use bpay::api::webhook::verification::{Verifier, Verify};
/// # use reqwest::header::HeaderMap;
/// # use bpay::errors::Result;
/// # use bpay::client::Client;
/// # #[tokio::main]
/// # async fn main() -> Result<()>{
/// # let headers = HeaderMap::new();
/// # let body = "";
/// # let client = Client::from_env();
/// let verifier = Verifier::from_api(&client).await?;
/// verifier.verify(&headers, &body).unwrap();
/// # Ok(())
/// # }
/// ```
impl Verifier {
    pub async fn from_api(client: &Client) -> Result<Self> {
        let certs = get_certificate(client).await?;
        Ok(Verifier::from(certs))
    }
}

#[cfg(test)]
mod tests {
    use super::refund_order::RefundOrder;
    use super::*;
    use crate::client::Client;
    use mockito;
    use mockito::mock;

    #[tokio::test]
    async fn test_query_order() {
        let url = &mockito::server_url();
        let response = r#"
     {
        "status":"SUCCESS",
        "code":"000000",
        "data":{
           "merchantId":98729382672,
           "prepayId":"383729303729303",
           "transactionId":"23729202729220282",
           "merchantTradeNo":"9825382937292",
           "tradeType":"APP",
           "status":"PAID",
           "currency":"EUR",
           "totalFee":10.88,
           "productName":"Ice Cream",
           "productDetail":"Greentea ice cream cone",
           "openUserId":"",
           "transactTime":1425744000123,
           "createTime":1425744000000
        },
        "errorMessage":""
     }
     "#;
        let client = Client::new(Some("".into()), Some("".into()), url.to_string());
        let _m = mock("POST", "/binancepay/openapi/order/query")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(response)
            .create();

        let qo = QueryOrder::new(None, Some("9825382937292".into()));
        let q_order_result = qo.post(&client).await.unwrap();
        assert_eq!(
            q_order_result.merchant_trade_no,
            "9825382937292".to_string()
        );
        assert_eq!(q_order_result.merchant_id, 98729382672)
    }

    #[tokio::test]
    async fn test_close_order() {
        let url = &mockito::server_url();
        let response = r#"
        {
            "status": "SUCCESS",
            "code": "000000",
            "data": true,
            "errorMessage": null
        }
        "#;
        let client = Client::new(Some("".into()), Some("".into()), url.to_string());
        let _m = mock("POST", "/binancepay/openapi/order/close")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(response)
            .create();

        let close_order = CloseOrder::new(None, Some("9825382937292".into()));
        let close_order_result = close_order.post(&client).await.unwrap();
        match close_order_result {
            CloseOrderResult(r) => assert!(r),
        }
    }

    #[tokio::test]
    async fn test_refund_order_deserialization() {
        let url = &mockito::server_url();
        let response = r#"
        {
            "status": "SUCCESS",
            "code": "000000",
            "data": {
              "refundRequestId": "68711039982968832",
              "prepayId": "383729303729303",
              "orderAmount": "100.11",
              "refundedAmount": "10.88",
              "refundAmount": "5.00",
              "remainingAttempts":8,
              "payerOpenId":"dde730c2e0ea1f1780cf26343b98fd3b",
              "duplicateRequest":"N"
            },
            "errorMessage": ""
        }
        "#;
        let client = Client::new(Some("".into()), Some("".into()), url.to_string());
        let _m = mock("POST", "/binancepay/openapi/order/refund")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(response)
            .create();

        let refund_order = RefundOrder {
            refund_request_id: "68711039982968832".into(),
            prepay_id: "383729303729303".into(),
            refund_amount: 25.17,
            refund_reason: Some("Upon Request".into()),
        };
        let refund_result = refund_order.post(&client).await.unwrap();
        assert_eq!(
            refund_result.refund_request_id,
            "68711039982968832".to_string()
        );
        match refund_result.duplicate_request {
            RefundDuplicateStatus::No => assert!(true),
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_wallet_balance_query() {
        let url = &mockito::server_url();
        let response = r#"
        {
            "status": "SUCCESS",
            "code": "000000",
            "data": {
              "balance": 990000.00000000,
              "asset": "BUSD",
              "fiat": "USD",
              "availableFiatValuation": 989991.90516600,
              "availableBtcValuation": 22.98780000
            }
        }
        "#;
        let client = Client::new(Some("".into()), Some("".into()), url.to_string());
        let _m = mock("POST", "/binancepay/openapi/balance")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(response)
            .create();

        let wallet_bal = WalletBalance {
            wallet: balance_query::WalletType::SpotWallet,
            currency: "BUSD".into(),
        };
        let wallet_bal = wallet_bal.post(&client).await.unwrap();
        assert_eq!(wallet_bal.available_fiat_valuation, 989991.90516600);
    }
}
