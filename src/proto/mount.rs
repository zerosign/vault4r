use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct MountConfigInfo {
    default_lease: usize,
    max_lease: usize,
    force_no_cache: bool,
    seal_wrap: bool,
}

#[derive(Debug, Serialize)]
pub enum Visibility {
    Unauth,
    Hidden
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MountConfig {
    inner: MountConfigInfo,
    request_keys: Vec<String>,
    response_keys: Vec<String>,
    visibility: Visibility,
    passthrough_headers: Vec<String>,
    allowed_response_headers: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MountInfo {
    r#type: String,
    description: String,
    config: MountConfigInfo,
}

#[derive(Debug, Serialize)]
pub struct Additional {
    local: bool,
    seal_wrap: bool,
}

#[derive(Debug, Serialize)]
pub struct Mount {
    path: String,
    r#type: String,
    description: String,
    config: MountConfig,
    options: HashMap<String, String>,
    version: String,
    additional: Option<Additional>,
}
