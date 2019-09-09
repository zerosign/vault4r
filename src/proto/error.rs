use serde_json::error::Error as JsonError;

#[derive(Debug)]
pub enum Error {
    HttpError(http::Error),
    JsonError(JsonError),
}

impl From<http::Error> for Error {
    fn from(e: http::Error) -> Self {
        Error::HttpError(e)
    }
}
