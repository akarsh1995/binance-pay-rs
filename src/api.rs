use reqwest::Url;

pub enum API {
    CreateOrder,
}

impl From<API> for String {
    fn from(item: API) -> Self {
        String::from(match item {
            CreateOrder => "/binancepay/openapi/v2/order",
        })
    }
}
