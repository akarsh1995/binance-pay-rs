use crate::api;
use crate::utils;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::header::InvalidHeaderValue;
use reqwest::header::CONTENT_TYPE;
use reqwest::Url;
use ring::hmac as rhmac;
use serde::de::DeserializeOwned;
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

    fn get_timestamp(&self) -> u128 {
        self.timestamp
    }

    fn get_nonce(&self) -> &str {
        self.nonce.as_str()
    }

    fn get_body(&self) -> &str {
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

    pub fn sign(&self, api_secret: &str) -> String {
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

    pub async fn post_signed<T: DeserializeOwned>(
        &self,
        endpoint: api::API,
        body: Option<String>,
    ) -> Result<T, reqwest::Error> {
        let request_content = RequestContent::from_body(body);
        let payload_signature = request_content.sign(&self.secret_key);
        let payload = request_content.get_body();
        let headers = self
            .get_header_map(
                request_content.get_timestamp(),
                request_content.get_nonce(),
                payload_signature.as_str(),
            )
            .unwrap();
        self.post(endpoint, payload.to_string(), headers).await
    }

    pub async fn post<T: DeserializeOwned>(
        &self,
        endpoint: api::API,
        body: String,
        headers: HeaderMap<HeaderValue>,
    ) -> Result<T, reqwest::Error> {
        let url = self.host.join(&String::from(endpoint)).unwrap();
        self.inner_client
            .post(url)
            .headers(headers)
            .body(body)
            .send()
            .await?
            .json()
            .await
    }

    fn get_header_map(
        &self,
        timestamp: u128,
        nonce: &str,
        signature: &str,
    ) -> Result<HeaderMap, InvalidHeaderValue> {
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
