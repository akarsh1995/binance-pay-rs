//! The module aims to provide the ease of use for the verification of the webhook request.
/*!
```rust,no_run
# use bpay::c2b::webhook::verification::{Verifier, Verify};
# use reqwest::header::HeaderMap;
# use bpay::errors::Result;
# fn main() -> Result<()>{
# let headers = HeaderMap::new();
# let body = "";
let verifier = Verifier {
    cert_serial: "Certificate's md5 hash goes here".to_string(),
    cert_public: "Certificate's public key goes here".to_string(),
};
verifier.verify(&headers, &body)?;
# Ok(())
# }
```
*/

use crate::errors::Error;
use crate::errors::Result;
use reqwest::header::HeaderMap;
use ring::signature;
use rsa_der::public_key_from_der;

/// Helper struct used to verify the signature of a request.
pub struct Verifier {
    pub cert_public: String,
    pub cert_serial: String,
}

impl<'a> Verifier {
    pub fn new(certificate: String, cert_serial: String) -> Self {
        Self {
            cert_public: certificate,
            cert_serial,
        }
    }
}

impl Verify<'_> for Verifier {
    fn get_certificate(&self) -> &str {
        &self.cert_public
    }

    fn get_certificate_serial(&self) -> &str {
        &self.cert_serial
    }
}

/// This trait needs to be implemented in order to verify the binance webhook request.
pub trait Verify<'a> {
    fn get_certificate(&self) -> &str;

    fn get_certificate_serial(&self) -> &str;

    fn get_timestamp(&self, headers: &'a HeaderMap) -> Result<&'a str> {
        self.get_header_val("binancepay-timestamp", headers)
    }

    fn get_nonce(&self, headers: &'a HeaderMap) -> Result<&'a str> {
        self.get_header_val("binancepay-nonce", headers)
    }

    fn get_public_key_md5(&self, headers: &'a HeaderMap) -> Result<&'a str> {
        self.get_header_val("binancepay-certificate-sn", headers)
    }

    fn get_signature(&self, headers: &'a HeaderMap) -> Result<&'a str> {
        self.get_header_val("binancepay-signature", headers)
    }

    fn get_header_val(&self, key: &str, headers: &'a HeaderMap) -> Result<&'a str> {
        Ok(headers
            .get(key)
            .expect(&format!("Could not find {key} in headers"))
            .to_str()?)
    }

    fn prep_payload(&self, headers: &'a HeaderMap, body: &str) -> Result<String> {
        Ok(format!(
            "{}\n{}\n{}\n",
            self.get_timestamp(headers)?,
            self.get_nonce(headers)?,
            body
        ))
    }

    fn get_decoded_signature(&self, headers: &'a HeaderMap) -> Result<Vec<u8>> {
        Ok(base64::decode(self.get_signature(headers)?)?)
    }

    /// The certificate obtained from the api is in the PEM format.
    /// This function decodes the PEM certificate contents to der format.
    fn get_decoded_certificate(&self) -> Result<Vec<u8>> {
        Ok(base64::decode(&self.get_parsed_certificate())?)
    }

    /// Extracts the public key from the der certificate format.
    fn get_public_key_from_der(&self) -> Result<(Vec<u8>, Vec<u8>)> {
        Ok(public_key_from_der(&self.get_decoded_certificate()?)?)
    }

    /// Verifies the signature of the request.
    fn verify(&self, headers: &'a HeaderMap, body: &str) -> Result<()> {
        assert_eq!(
            self.get_public_key_md5(headers)?,
            self.get_certificate_serial()
        );
        let der_decoded = self.get_public_key_from_der()?;
        Ok(signature::RsaPublicKeyComponents {
            n: der_decoded.0,
            e: der_decoded.1,
        }
        .verify(
            &signature::RSA_PKCS1_2048_8192_SHA256,
            self.prep_payload(headers, body)?.as_bytes(),
            &self.get_decoded_signature(headers)?,
        )
        .map_err(|_| Error::Msg("Signature verification failed".into()))?)
    }

    /// Get the PEM certificate contents.
    fn get_parsed_certificate(&self) -> String {
        self.get_certificate()
            .split('\n')
            .filter(|line| !line.starts_with("-----"))
            .collect::<Vec<&str>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderValue;
    use ring::rand;
    use ring::signature;
    use serde::Deserialize;
    use std::fs::read_to_string;

    #[derive(Deserialize, Debug)]
    pub struct TestData {
        private_key: String,
        cert_public: String,
        cert_public_md5_hash: String,
    }

    pub fn read_test_data() -> TestData {
        let raw_data = read_to_string("data/key_pair.json").unwrap();
        serde_json::from_str(&raw_data).unwrap()
    }

    pub(crate) fn get_encoded_signature(
        nonce: &str,
        body: &str,
        timestamp: &str,
        private_key: &str,
    ) -> String {
        let payload = format!("{}\n{}\n{}\n", timestamp, nonce, body);
        let parsed_private_key = private_key
            .split('\n')
            .filter(|line| !line.starts_with("-----"))
            .collect::<Vec<&str>>()
            .join("");

        let decoded_priv_key = base64::decode(parsed_private_key).unwrap();
        let private_key_from_der = signature::RsaKeyPair::from_der(&decoded_priv_key).unwrap();
        let rng = rand::SystemRandom::new();
        let mut signature = vec![0; private_key_from_der.public_modulus_len()];
        private_key_from_der
            .sign(
                &signature::RSA_PKCS1_SHA256,
                &rng,
                payload.as_bytes(),
                &mut signature,
            )
            .unwrap();
        base64::encode(signature)
    }

    #[test]
    fn check_signature_verification() {
        let timestamp = 1646584911979 as u128;
        let timestamp = timestamp.to_string();
        let nonce = "NldzYKVJuiwjCHQGlaZfwnbGaFLPimYH";
        let body = r#"{"env":{"terminalType":"WEB"},"merchantTradeNo":"9825382937292","orderAmount":25.0,"currency":"USDT","goods":{"goodsType":"02","goodsCategory":"D000","referenceGoodsId":"7876763A3B","goodsName":"Ice Cream","goodsDetail":"Greentea ice cream cone"}}"#;
        let malformed_body = "malformed_body";
        let test_data = read_test_data();
        let mut headers: HeaderMap = HeaderMap::new();
        let header_keys = [
            "BinancePay-Timestamp",
            "BinancePay-Nonce",
            "BinancePay-Certificate-SN",
            "BinancePay-Signature",
        ];
        let encoded_signature =
            get_encoded_signature(nonce, body, &timestamp.to_string(), &test_data.private_key);
        let header_vals = [
            timestamp.as_str(),
            nonce,
            &test_data.cert_public_md5_hash,
            &encoded_signature,
        ];

        for (key, val) in header_keys.iter().zip(header_vals.iter()) {
            headers.insert(*key, HeaderValue::from_str(*val).unwrap());
        }
        let v = Verifier {
            cert_serial: test_data.cert_public_md5_hash,
            cert_public: test_data.cert_public,
        };
        v.verify(&headers, body).unwrap();
        let occured = v.verify(&headers, malformed_body).unwrap_err();
        match occured {
            Error::Msg(e) => assert_eq!(e, "Signature verification failed"),
            _ => assert!(false),
        }
    }
}
