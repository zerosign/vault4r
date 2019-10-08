use crate::proto::error::Error;
use http::Request;
use hyper::Body;
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

pub trait SealEndpoint {
    const SEAL_ENDPOINT: &'static str = "/sys/seal";
    const UNSEAL_ENDPOINT: &'static str = "/sys/unseal";
    const SEAL_INFO_ENDPOINT: &'static str = "/sys/seal-status";

    // https://www.vaultproject.io/api/system/seal.html
    fn seal(&self) -> Result<Request<Body>, Error>;

    // https://www.vaultproject.io/api/system/unseal.html
    fn unseal(
        &self,
        key: String,
        reset: Option<bool>,
        migrate: Option<bool>,
    ) -> Result<Request<Body>, Error>;

    // https://www.vaultproject.io/api/system/seal-status.html
    fn seal_info(&self) -> Result<Request<Body>, Error>;
}
