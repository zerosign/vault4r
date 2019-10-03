use serde::de::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ClusterInfo {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct SealInfo {
    r#type: String,
    threshold: usize,
    shares: usize,
    progress: usize,
    nonce: String,
    version: String,
}

#[derive(Debug, Deserialize)]
pub enum Status {
    Sealed {
        info: SealInfo,
    },
    Unsealed {
        info: SealInfo,
        cluster: ClusterInfo,
    },
}
