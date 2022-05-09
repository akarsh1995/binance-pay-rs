//! Contains all webhook related helpers.

pub mod notification;
pub mod verification;

pub mod certificate {
    //! Contains deserailizable formats of the certificate's request and response.

    use serde::{Deserialize, Serialize};

    use super::verification::Verifier;

    /// Empty body of the query certificate request.
    #[derive(Serialize, Debug)]
    pub struct Certificate;

    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct CertificateResult {
        /// Public key md5 hash value.
        pub cert_serial: String,

        /// Public key in PEM format.
        pub cert_public: String,
    }

    impl<'a> From<CertificateResult> for Verifier {
        fn from(cert_result: CertificateResult) -> Self {
            Self::new(cert_result.cert_public, cert_result.cert_serial)
        }
    }
}
