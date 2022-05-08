pub use crate::c2b::*;
use crate::client;
use crate::errors::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

pub enum API {
    CreateOrder,
}

impl From<API> for String {
    fn from(item: API) -> Self {
        String::from(match item {
            API::CreateOrder => "/binancepay/openapi/v2/order",
        })
    }
}

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
    async fn post(&self, client: client::Client) -> Result<D> {
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
