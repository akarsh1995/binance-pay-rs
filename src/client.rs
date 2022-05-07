use crate::api;
use crate::errors::BinanceContentError;
use crate::errors::Error;
use crate::errors::Result;
use crate::utils;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::header::CONTENT_TYPE;
use reqwest::Response;
use reqwest::StatusCode;
use reqwest::Url;
use ring::hmac as rhmac;
use serde::de::DeserializeOwned;
use serde_json::from_str;
use std::str::FromStr;

pub struct Client {
    api_key: String,
    secret_key: String,
    host: Url,
    inner_client: reqwest::Client,
}

struct RequestContent {
    timestamp: u128,
    nonce: String,
    body: Option<String>,
}

impl Default for RequestContent {
    fn default() -> Self {
        Self {
            timestamp: utils::get_current_timestamp(),
            nonce: utils::create_nonce(32),
            body: None,
        }
    }
}

impl RequestContent {
    pub fn from_body(body: Option<String>) -> Self {
        Self {
            body,
            ..Default::default()
        }
    }

    pub fn get_timestamp(&self) -> u128 {
        self.timestamp
    }

    pub fn get_nonce(&self) -> &str {
        self.nonce.as_str()
    }

    pub fn get_body(&self) -> &str {
        match &self.body {
            Some(body) => body.as_str(),
            None => "",
        }
    }

    fn signature_payload(&self) -> String {
        format!(
            "{}\n{}\n{}\n",
            self.get_timestamp(),
            self.get_nonce(),
            self.get_body(),
        )
    }

    fn sign(&self, api_secret: &str) -> String {
        let key = rhmac::Key::new(rhmac::HMAC_SHA512, api_secret.as_bytes());
        let raw_signature = rhmac::sign(&key, self.signature_payload().as_bytes());
        hex::encode_upper(raw_signature.as_ref())
    }
}

impl Client {
    /// Returns a client based on the specified host and credentials
    /// Credentials do not need to be specified when using public endpoints
    /// Host is mandatory
    pub fn new(api_key: Option<String>, secret_key: Option<String>, host: String) -> Self {
        Self {
            api_key: api_key.unwrap_or_else(|| "".into()),
            secret_key: secret_key.unwrap_or_else(|| "".into()),
            host: Url::from_str(&host).unwrap(),
            inner_client: reqwest::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .unwrap(),
        }
    }

    pub async fn post_signed_de<T: DeserializeOwned>(
        &self,
        endpoint: api::API,
        request: Option<String>,
    ) -> Result<T> {
        let r = self.post_signed(endpoint, request).await?;
        let t = from_str(&r)?;
        Ok(t)
    }

    pub async fn post_signed_s<T: DeserializeOwned, S: serde::Serialize>(
        &self,
        endpoint: api::API,
        serializable: Option<S>,
    ) -> Result<T> {
        let request_str: String = if let Some(serializable) = serializable {
            serde_json::to_string(&serializable)?
        } else {
            "".to_string()
        };
        self.post_signed_de(endpoint, Some(request_str)).await
    }

    pub async fn post_signed(&self, endpoint: api::API, request: Option<String>) -> Result<String> {
        let request_content = RequestContent::from_body(request);
        let payload_signature = request_content.sign(&self.secret_key);
        let payload = request_content.get_body();
        let headers = self.build_headers(
            request_content.get_timestamp(),
            request_content.get_nonce(),
            payload_signature.as_str(),
        )?;
        self.post(endpoint, payload.to_string(), headers).await
    }

    pub async fn post(
        &self,
        endpoint: api::API,
        body: String,
        headers: HeaderMap<HeaderValue>,
    ) -> Result<String> {
        let url = self.host.join(&String::from(endpoint)).unwrap();
        let response = self
            .inner_client
            .post(url)
            .headers(headers)
            .body(body)
            .send()
            .await?;
        self.handler(response).await
    }

    async fn handler(&self, response: Response) -> Result<String> {
        match response.status() {
            StatusCode::OK => {
                let body = response.bytes().await?;
                let result = std::str::from_utf8(&body);
                Ok(result?.to_string())
            }
            StatusCode::INTERNAL_SERVER_ERROR => Err(Error::InternalServerError),
            StatusCode::SERVICE_UNAVAILABLE => Err(Error::ServiceUnavailable),
            StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),
            StatusCode::BAD_REQUEST => {
                let error: BinanceContentError = response.json().await?;
                Err(match (error.code, &error.error_message) {
                    _ => Error::BinanceError { response: error },
                })
            }
            s => Err(Error::Msg(format!("Received response: {:?}", s))),
        }
    }

    fn build_headers(&self, timestamp: u128, nonce: &str, signature: &str) -> Result<HeaderMap> {
        let header_keys = [
            "BinancePay-Timestamp",
            "BinancePay-Nonce",
            "BinancePay-Certificate-SN",
            "BinancePay-Signature",
        ];
        let timestamp = timestamp.to_string();
        let header_vals = [timestamp.as_str(), nonce, self.api_key.as_str(), signature];
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
        for i in 0..header_keys.len() {
            headers.insert(header_keys[i], HeaderValue::from_str(header_vals[i])?);
        }
        Ok(headers)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_signature_algo() {
        let secret_key = "abcd1234";
        let rc = RequestContent {
            timestamp: 1636876819,
            nonce: "abcd".to_string(),
            body: Some("akarshjain".to_string()),
        };
        assert_eq!(&rc.sign(secret_key), "0FEE450C836654F95CA8AC5B99DB385B96CAE1EDC46456A5BEA005BFA020FC113AD61D9B8595BA951A3A562BBB8556B6D063D6BA8AEF488097642E50ACC27ACA")
    }
}
