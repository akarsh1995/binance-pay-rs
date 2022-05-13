//! Use Binance [`client::Client`] in conjunction with the Request Response structs.
//! - [`create_order::Order`] and [`create_order::CreateOrderResult`]

use self::webhook::certificate::{Certificate, CertificateResult};
use crate::c2b::webhook::verification::Verifier;
pub use crate::c2b::*;
use crate::client;
use crate::client::Client;
use crate::errors::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub enum API {
    CreateOrder,
    QueryCertificate,
    QueryOrder,
    CloseOrder,
    RefundOrder,
    BalanceQuery,
    TransferFund,
    QueryTransfer,
    QueryRefund,
    BatchPayout,
    CreateSubMerchant,
    PayoutQuery,
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
            API::TransferFund => "/binancepay/openapi/wallet/transfer",
            API::QueryTransfer => "/binancepay/openapi/wallet/transfer/query",
            API::QueryRefund => "/binancepay/openapi/order/refund/query",
            API::BatchPayout => "/binancepay/openapi/payout/transfer",
            API::CreateSubMerchant => "/binancepay/openapi/submerchant/add",
            API::PayoutQuery => "/binancepay/openapi/payout/query",
        })
    }
}

/// Response format from the Binance Pay API.
#[derive(Deserialize)]
struct Response<T> {
    data: T,
}

#[async_trait::async_trait]
pub trait Binance<D>: Serialize + Sized
where
    D: DeserializeOwned,
{
    #[deprecated(
        since = "0.3.2",
        note = "Use respective action method instead, e.g. .create(), .query(), etc."
    )]
    async fn post(&self, client: &client::Client) -> Result<D> {
        let response = client
            .post_signed_s::<Response<D>, Self>(self.get_api().into(), Some(self))
            .await?;
        Ok(response.data)
    }

    fn get_api(&self) -> API;
}

/// implelments the trait [`Binance`] for all the requests.
/// follows pattern (moule::{action}, api_endpoint)
macro_rules! impl_binance {
    (
        $(($x: ident::$y: ident, $z: ident)),*
    ) => {
        $(
            impl Binance<$x::$y::Response> for $x::$y::Request {
                fn get_api(&self) -> API {
                    API::$z
                }
            }

            impl $x::$y::Request {
                pub async fn $y(&self, client: &Client) -> Result<$x::$y::Response>  {
                    let response = client
                        .post_signed_s::<Response<$x::$y::Response>, Self>(self.get_api().into(), Some(self))
                        .await?;
                    Ok(response.data)
                }
            }
        )*
    };
}

impl_binance!(
    (order::create, CreateOrder),
    (order::query, QueryOrder),
    (order::close, CloseOrder),
    (refund::initiate, RefundOrder),
    (wallet_balance::query, BalanceQuery),
    (transfer::initiate, TransferFund),
    (transfer::query, QueryTransfer),
    (refund::query, QueryRefund),
    (payout::initiate, BatchPayout),
    (sub_merchant::create, CreateSubMerchant),
    (payout::query, PayoutQuery)
);

impl Binance<CertificateResult> for Certificate {
    fn get_api(&self) -> API {
        API::QueryCertificate
    }
}

impl Certificate {
    pub async fn fetch(&self, client: &Client) -> Result<CertificateResult> {
        let mut response = client
            .post_signed_s::<Response<Vec<CertificateResult>>, Certificate>(
                API::QueryCertificate.into(),
                Some(self),
            )
            .await?;
        Ok(response
            .data
            .pop()
            .expect("Couldn't find the certificate inside the response array"))
    }
}

/// Get certificate out of the received response array.
pub async fn get_certificate(client: &Client) -> Result<CertificateResult> {
    Ok(Certificate.fetch(client).await?)
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
