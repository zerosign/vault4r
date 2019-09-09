use std::time;

#[derive(Debug, Deserialize)]
pub struct Response {
    term: usize,
    install_time: time::Instant,
}
