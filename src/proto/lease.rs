use crate::proto::error::Error;
use hyper::{Body, Request};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum LeaseStatus {
    Renewable(Status),
    Static(Status),
}

#[derive(Debug, Deserialize)]
pub struct Status {
    id: String,
    issue_time: usize,
    expire_time: usize,
    last_renewal_time: Option<usize>,
    ttl: usize,
}

pub trait LeaseEndpoint {
    const READ_LEASE_ENDPOINT: &'static str = "/sys/leases/lookup";

    // read_lease
    fn read_lease<S>(&self, id: S) -> Result<Request<Body>, Error>
    where
        S: Into<String>;

    // list_lease
    // renew_lease
    // revoke_lease
}
