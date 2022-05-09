//! Use Binance [`client::Client`] in conjunction with the Request Response structs.
//! - [`create_order::Order`] and [`create_order::CreateOrderResult`]

use self::webhook::certificate::{Certificate, CertificateResult};
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
}

impl From<API> for String {
    fn from(item: API) -> Self {
        String::from(match item {
            API::CreateOrder => "/binancepay/openapi/v2/order",
            API::QueryCertificate => "/binancepay/openapi/certificates",
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
