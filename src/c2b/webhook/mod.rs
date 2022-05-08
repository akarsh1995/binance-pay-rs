pub mod certificate {

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Debug)]
    pub struct Certificate;

    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct CertificateResult {
        pub cert_serial: String,

        pub cert_public: String,
    }
}
