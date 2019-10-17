use hyper::Error as HyperError;
use serde_json::Error as SerdeError;
use std::convert::From;
use vaultr_proto::error::Error as ProtoError;

#[derive(Debug)]
pub enum Error {
    ProtoError(ProtoError),
    HyperError(HyperError),
    SerdeError(SerdeError),
}

impl From<ProtoError> for Error {
    #[inline]
    fn from(e: ProtoError) -> Self {
        Self::ProtoError(e)
    }
}

impl From<HyperError> for Error {
    #[inline]
    fn from(e: HyperError) -> Self {
        Self::HyperError(e)
    }
}

impl From<SerdeError> for Error {
    #[inline]
    fn from(e: SerdeError) -> Self {
        Self::SerdeError(e)
    }
}
