//! Use Binance [`client::Client`] in conjunction with the Request Response structs.
//! - [`create_order::Order`] and [`create_order::CreateOrderResult`]

use self::{
    close_order::{CloseOrder, CloseOrderResult},
    query_order::{QueryOrder, QueryOrderResult},
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
}

impl From<API> for String {
    fn from(item: API) -> Self {
        String::from(match item {
            API::CreateOrder => "/binancepay/openapi/v2/order",
            API::QueryCertificate => "/binancepay/openapi/certificates",
            API::QueryOrder => "/binancepay/openapi/order/query",
            API::CloseOrder => "/binancepay/openapi/order/close",
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

impl Binance<create_order::CreateOrderResult> for create_order::Order {
    fn get_api(&self) -> API {
        API::CreateOrder
    }
}

impl Binance<Vec<CertificateResult>> for Certificate {
    fn get_api(&self) -> API {
        API::QueryCertificate
    }
}

/// Get certificate out of the received response array.
pub async fn get_certificate(client: &Client) -> Result<CertificateResult> {
    let mut certs_arr = Certificate.post(client).await?;
    Ok(certs_arr.pop().expect("No certificates found"))
}

/// Get [`Verifier`] directly from the api.
impl Verifier {
    pub async fn from_api(client: &Client) -> Result<Self> {
        let certs = get_certificate(client).await?;
        Ok(Verifier::from(certs))
    }
}

impl Binance<QueryOrderResult> for QueryOrder {
    fn get_api(&self) -> API {
        API::QueryOrder
    }
}

impl Binance<CloseOrderResult> for CloseOrder {
    fn get_api(&self) -> API {
        API::CloseOrder
    }
}

#[cfg(test)]
mod tests {
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
}
