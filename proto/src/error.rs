use http::Error as HttpError;
use serde_json::error::Error as JsonError;
use std::convert::From;

///
/// Error for proto builder.
///
/// This include [JsonError](JsonError) or [HttpError](HttpError).
///
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
