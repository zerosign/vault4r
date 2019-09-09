use crate::proto::error::Error;
use http::Request;
use hyper::Body;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
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

pub trait HealthEndpoint {
    const HEALTH_ENDPOINT: &'static str = "/sys/health";

    fn health(&self) -> Result<Request<Body>, Error>;
}
