use crate::error::Error;
use futures::TryFuture;
use http::Request;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub enum LeaseStatus {
    Renewable(Status),
    Static(Status),
}

#[derive(Debug, Deserialize, Default)]
pub struct Status {
    id: String,
    issue_time: usize,
    expire_time: usize,
    last_renewal_time: Option<usize>,
    ttl: usize,
}

impl Default for LeaseStatus {
    fn default() -> Self {
        LeaseStatus::Static(Status::default())
    }
}

pub trait LeaseEndpoint<Payload>
where
    Payload: Send + 'static,
{
    const READ_LEASE_ENDPOINT: &'static str = "/sys/leases/lookup";
    const RENEW_LEASE_ENDPOINT: &'static str = "/sys/leases/renew";
    const REVOKE_LEASE_ENDPOINT: &'static str = "/sys/leases/revoke";
    const REVOKE_LEASE_FORCE_ENDPOINT: &'static str = "/sys/leases/revoke-force";
    const REVOKE_LEASE_PREFIX_ENDPOINT: &'static str = "/sys/leases/revoke-prefix";

    // read_lease
    fn read_lease(&self, id: &str) -> Result<Request<Payload>, Error>;

    // list_lease
    fn list_lease(&self, prefix: Option<&str>) -> Result<Request<Payload>, Error>;

    // renew_lease
    fn renew_lease(&self, id: &str, duration: Duration) -> Result<Request<Payload>, Error>;

    // revoke_lease
    fn revoke_lease(&self, id: &str) -> Result<Request<Payload>, Error>;

    // revoke_prefix
    fn revoke_prefix(&self, prefix: &str, forced: bool) -> Result<Request<Payload>, Error>;
}

pub trait LeaseService {
    type LeaseError;

    type LeaseInfoFuture: TryFuture<Ok = LeaseStatus, Error = Self::LeaseError> + 'static;
    type ListLeaseFuture: TryFuture<Ok = Vec<String>, Error = Self::LeaseError> + 'static;
    type RevokedLeaseFuture: TryFuture<Ok = (), Error = Self::LeaseError> + 'static;
    type RevokedPrefixFuture: TryFuture<Ok = (), Error = Self::LeaseError> + 'static;

    fn read_lease(&self, id: &str) -> Self::LeaseInfoFuture;

    fn list_lease(&self, prefix: Option<&str>) -> Self::ListLeaseFuture;

    fn revoke_lease(&self, id: &str) -> Self::RevokedLeaseFuture;

    fn revoke_prefix(&self, prefix: &str, forced: bool) -> Self::RevokedPrefixFuture;
}
