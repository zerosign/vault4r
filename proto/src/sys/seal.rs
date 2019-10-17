use crate::error::Error;
use futures::TryFuture;
use http::Request;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ClusterInfo {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct SealInfo {
    r#type: String,
    threshold: usize,
    shares: usize,
    progress: usize,
    nonce: String,
    version: String,
}

#[derive(Debug, Deserialize)]
pub enum SealStatus {
    Sealed {
        info: SealInfo,
    },
    Unsealed {
        info: SealInfo,
        cluster: ClusterInfo,
    },
}

pub trait SealEndpoint<Payload>
where
    Payload: Send + 'static,
{
    const SEAL_ENDPOINT: &'static str = "/sys/seal";
    const UNSEAL_ENDPOINT: &'static str = "/sys/unseal";
    const SEAL_INFO_ENDPOINT: &'static str = "/sys/seal-status";

    // https://www.vaultproject.io/api/system/seal.html
    fn seal(&self) -> Result<Request<Payload>, Error>;

    // https://www.vaultproject.io/api/system/unseal.html
    fn unseal(
        &self,
        key: String,
        reset: Option<bool>,
        migrate: Option<bool>,
    ) -> Result<Request<Payload>, Error>;

    // https://www.vaultproject.io/api/system/seal-status.html
    fn seal_info(&self) -> Result<Request<Payload>, Error>;
}

pub trait SealService {
    type SealError;

    type SealFuture: TryFuture<Ok = (), Error = Self::SealError> + 'static;
    type UnsealFuture: TryFuture<Ok = SealStatus, Error = Self::SealError> + 'static;
    type SealInfoFuture: TryFuture<Ok = SealStatus, Error = Self::SealError> + 'static;

    fn seal(&self) -> Self::SealFuture;

    fn unseal(&self, key: String, reset: Option<bool>, migrate: Option<bool>)
        -> Self::UnsealFuture;

    fn seal_info(&self) -> Self::SealInfoFuture;
}
