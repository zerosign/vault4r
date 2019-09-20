#[derive(Debug)]
pub struct AuthInfo {
    r#type: String,
    description: String,
    config: Option<String>,
}
