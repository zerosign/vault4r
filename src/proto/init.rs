#[derive(Debug, Serialize)]
pub struct Init {
    keys: Option<Vec<[u8]>>,
    root_token: String,
    threshold: usize,
}

#[derive(Debug, Serialize)]
pub struct RecoveryKey {
    shares: usize,
    threshold: usize,
    keys: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SysInitPro {
    inner: SysInit,
    recovery: RecoveryKey,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    keys: Vec<String>,
    keys_base64: Vec<String>,
    root_token: String,
}
