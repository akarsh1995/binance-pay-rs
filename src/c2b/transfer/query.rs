//! Query Transfer Result API used for merchant/partner to query transfer result.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct QueryTransferRequest {
    /// the value of requestId of provoking Transfer Fund API
    pub tran_id: String,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, derive(Serialize))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Success,
    Failure,
    Process,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, derive(Serialize))]
pub struct QueryTransferResult {
    ///	string	Y	-	Used to query the transfer status, query the necessary fields for the transfer status
    pub tran_id: String,
    /// 	string	Y		SUCCESS (indicating that the transfer is completely successful), FAILURE (indicating that the transfer has failed, it may be that the transferor has a problem with the transferee), PROCESS (the transfer is in progress)
    pub status: Status,
}

#[cfg(test)]
mod tests {
    use crate::c2b::tests::test_request_serialize_deserialize;

    test_request_serialize_deserialize!(
        (
            test_query_transfer_request_serialize,
            r#"{"tran_id":"100002021071407140001"}"#,
            QueryTransferRequest {
                tran_id: "100002021071407140001".to_string(),
            }
        ),
        (
            test_query_transfer_result_deserialize,
            r#"{"tran_id":"100002021071407140001","status":"SUCCESS"}"#,
            QueryTransferResult {
                tran_id: "100002021071407140001".to_string(),
                status: Status::Success,
            }
        )
    );
}
