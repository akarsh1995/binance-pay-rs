//! Use Binance [`client::Client`] in conjunction with the Request Response structs.
//! - [`create_order::Order`] and [`create_order::CreateOrderResult`]

use self::{
    balance_query::{WalletBalance, WalletBalanceResult},
    create_sub_merchant::{SubMerchantRequest, SubMerchantResult},
    query_transfer::{QueryTransferRequest, QueryTransferResult},
    transfer_fund::{TransferFund, TransferFundResult},
    webhook::certificate::{Certificate, CertificateResult},
};
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
    async fn post(&self, client: &client::Client) -> Result<D> {
        let response = client
            .post_signed_s::<Response<D>, Self>(self.get_api().into(), Some(self))
            .await?;
        Ok(response.data)
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
    (order::create::Response, order::create::Request, CreateOrder),
    (Vec<CertificateResult>, Certificate, QueryCertificate),
    (
        order::query::QueryOrderResult,
        order::query::QueryOrder,
        QueryOrder
    ),
    (order::close::Response, order::close::Request, CloseOrder),
    (
        refund::create::RefundResult,
        refund::create::RefundOrder,
        RefundOrder
    ),
    (WalletBalanceResult, WalletBalance, BalanceQuery),
    (TransferFundResult, TransferFund, TransferFund),
    (QueryTransferResult, QueryTransferRequest, QueryTransfer),
    (
        refund::query::QueryRefundResult,
        refund::query::QueryRefundRequest,
        QueryRefund
    ),
    (
        payout::initiate::Response,
        payout::initiate::Request,
        BatchPayout
    ),
    (SubMerchantResult, SubMerchantRequest, CreateSubMerchant),
    (payout::query::Response, payout::query::Request, PayoutQuery)
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
