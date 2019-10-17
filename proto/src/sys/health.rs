use crate::error::Error;
use http::Request;
use serde::Deserialize;
use futures::TryFuture;

#[derive(Debug, PartialEq, Deserialize, Default)]
pub struct HealthInfo {
    initialized: bool,
    sealed: bool,
    standby: bool,
    performance_standby: bool,
    replication_perf_mode: String,
    replication_dr_mode: String,
    server_time: usize,
    version: String,
    cluster_name: String,
    cluster_id: String,
}

pub trait HealthEndpoint<Payload> where Payload : Send + 'static {
    const HEALTH_ENDPOINT: &'static str = "/sys/health";

    fn health(&self) -> Result<Request<Payload>, Error>;
}

pub trait HealthService {
    type HealthError;

    //
    //
    type HealthFuture: TryFuture<Ok = HealthInfo, Error = Self::HealthError> + 'static;

    //
    // request : GET (version)/sys/health
    //
    fn health(&self) -> Self::HealthFuture;
}
