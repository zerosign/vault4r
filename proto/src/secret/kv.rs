use crate::error::Error;
use futures::TryFuture;
use http::Request;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Value<T> {
    auth: Option<&str>,
    data: T,
    lease: (usize, &str),
    renewable: bool,>
}

///
/// [Modify Secret Config Payload](https://www.vaultproject.io/api/secret/kv/kv-v2.html#sample-payload)
///
#[derive(Debug, Deserialize)]
pub struct Config {
    /// cas_required
    #[serde(rename(deserialize = "cas_required"))]
    atomic: bool,
    /// max_version
    #[serde(rename(deserialize = "max_version"))]
    version: usize,
    /// delete_version_after
    #[serde(rename(deserialize = "delete_version_after"))]
    timeout: String,
}

pub trait KeyValueEndpoint<Payload>
where
    Payload: Send + 'static,
{
    fn read_kv(&self, path: &str) -> Result<Request<Payload>, Error>;

    fn list_kv(&self, path: &str) -> Result<Request<Payload>, Error>;

    fn create_kv(&self, path: &str) -> Result<Request<Payload>, Error>;

    fn update_kv(&self, path: &str) -> Result<Request<Payload>, Error>;
}

pub trait KeyValueConfig<Payload>
where Payload: Send + 'static {
    fn config_kv(&self, path: &str) -> Result<Request<Payload>, Error>;
    fn modify_kv_config(&self, path: &str) -> Result<Request<Payload>, Error>;
}

pub trait KeyValueConfigService {
    type KeyValueConfigError;

    type KvConfigFuture: TryFuture<Ok = Config, Error = Self::KeyValueConfigError>;
    type KvUpdateConfigFuture: TryFuture<Ok = (), Error = Self::KeyValueConfigError>;

    fn config_kv(&self, path: &str) -> Self::KvConfigFuture;
    fn modify_kv_config(&self, path: &str) -> Self::KvUpdateConfigFuture;
}

pub trait KeyValueService {
    type KeyValueError;

    type ReadKvFuture: TryFuture<Ok = Value<Secret>, Error = Self::KeyValueError>;
    type ListKvFuture: TryFuture<Ok = Value<Vec<&str>>, Error = Self::KeyValueError>;
    type CreateKvFuture: TryFuture<Ok = (), Error = Self::KeyValueError>;
    type UpdateKvFuture: TryFuture<Ok = (), Error = Self::KeyValueError>;

    fn read_kv(&self, path: &str) -> Self::ReadKvFuture;
    fn list_kv(&self, path: &str) -> Self::ListKvFuture;
    fn create_kv(&self, path: &str) -> Self::CreateKvFuture;
    fn update_kv(&self, path: &str) -> Self::UpdateKvFuture;
}
