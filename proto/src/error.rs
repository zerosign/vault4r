use std::convert::From;
use http::Error as HttpError;
use serde_json::error::Error as JsonError;

#[derive(Debug)]
pub enum Error {
    HttpError(HttpError),
    JsonError(JsonError),
}

impl From<HttpError> for Error {
    #[inline]
    fn from(e: HttpError) -> Self {
        Self::HttpError(e)
    }
}

impl From<JsonError> for Error {
    #[inline]
    fn from(e: JsonError) -> Self {
        Self::JsonError(e)
    }
}
