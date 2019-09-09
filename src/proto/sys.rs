use http::Request;

pub trait HealthEndpoint {
    const HEALTH_ENDPOINT: &'static str = "/sys/health";
    fn health(&self) -> http::Result<Request<()>>;
}
