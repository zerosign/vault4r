use std::time::Instant;

#[derive(Debug, Deserialize)]
pub struct LicenseInfo {
    expired_time: time::Instant,
    features: Vec<String>,
    license_id: String,
    start_time: time::Instant,
}
