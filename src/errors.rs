use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "camelCase")]
#[error("code: {code}, error_message: {error_message}")]
pub struct BinanceContentError {
    pub status: String,
    pub code: i16,
    pub error_message: String,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReqError(#[from] reqwest::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    InvalidHeaderError(#[from] reqwest::header::InvalidHeaderValue),

    #[error(transparent)]
    UTF8Err(#[from] std::str::Utf8Error),

    #[error("{response}")]
    BinanceError {
        #[from]
        response: BinanceContentError,
    },

    #[error("internal server error")]
    InternalServerError,

    #[error("service unavailable")]
    ServiceUnavailable,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("{0}")]
    Msg(String),
}

pub type Result<T> = core::result::Result<T, Error>;
