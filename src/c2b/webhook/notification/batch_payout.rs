//! Order notification json deserialization format.

pub use crate::c2b::payout::query::BatchStatus as BizStatus;
use serde::Deserialize;

#[cfg(test)]
use serde::Serialize;

#[derive(Deserialize, Debug)]
#[cfg_attr(test, derive(Serialize))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    /// Accepted the request, will process it soon.  
    Accepted,

    /// Batch under processing
    Processing,

    /// All transfers are successful
    Success,

    /// Transfers partially succeeded
    PartSuccess,

    /// All transfers failed
    Failed,

    /// There remains transfers with unknown results after retry limit, will not retry further
    Canceled,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(test, derive(Serialize))]
#[serde(rename_all = "camelCase")]
pub struct Payout {
    /// The passed-in request ID
    pub request_id: String,

    pub batch_status: Status,

    pub merchant_id: u64,

    /// The request currency.
    pub currency: String,

    /// Total transfer amount in this batch.
    pub total_amount: f64,

    /// Total number of transfers in the batch.
    pub total_number: u16,
}

#[cfg(test)]
mod tests {
    use crate::c2b::tests::test_request_serialize_deserialize;

    test_request_serialize_deserialize!(
        (
            test_batch_payout_notification_serialize,
            "{\"batchStatus\":\"SUCCESS\",\"currency\":\"BUSD\",\"merchantId\":100100006288,\"requestId\":\"gg8127129\",\"totalAmount\":2.00000000,\"totalNumber\":2}",
            Payout {
                request_id: "gg8127129".to_string(),
                batch_status: Status::Success,
                merchant_id: 100100006288,
                currency: "BUSD".to_string(),
                total_amount: 2.0,
                total_number: 2,
            }
        )
    );
}
