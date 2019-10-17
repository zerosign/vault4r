use crate::error::Error;
use http::Request;
use futures::TryFuture;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct KeysInfo {
    keys: Vec<String>,
    keys_base64: Vec<String>,
    root_token: String
}

pub trait InitEndpoint<Payload> where Payload : Send + 'static {
    const INIT_ENDPOINT: &'static str = "/sys/init";

    fn init_status(&self) -> Result<Request<Payload>, Error>;

    fn initialize(&self, keys: Vec<String>, token: String, shares: usize, threshold: usize) -> Result<Request<Payload>, Error>;
}

pub trait InitService {
    type InitError;

    type InitStatusFuture: TryFuture<Ok = bool, Error = Self::InitError> + 'static;
    type InitializeFuture: TryFuture<Ok = KeysInfo, Error = Self::InitError> + 'static;

    fn init_status(&self) -> Self::InitStatusFuture;
    fn initialize(&self, keys: Vec<String>, token: String, shares: usize, threshold: usize) -> Self::InitializeFuture;
}
