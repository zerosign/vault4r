use crate::proto::error::Error;
use hyper::{Body, Request};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MountConfigInfo {
    default_lease: usize,
    max_lease: usize,
    force_no_cache: bool,
    seal_wrap: bool,
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

#[derive(Debug, Serialize, Deserialize)]
pub enum Visibility {
    Hidden,
    Unauth,
}

#[derive(Debug, Serialize)]
pub struct KeyPairs {
    request: Vec<String>,
    response: Vec<String>,
}

impl Default for KeyPairs {
    #[inline]
    fn default() -> Self {
        KeyPairs {
            request: Vec::with_capacity(0),
            response: Vec::with_capacity(0),
        }
    }
}

pub trait MountEndpoint {
    const MOUNTS_ENDPOINT: &'static str = "/sys/mounts";

    // https://www.vaultproject.io/api/system/mounts.html#list-mounted-secrets-engines
    fn list_mounts(&self) -> Result<Request<Body>, Error>;

    // https://www.vaultproject.io/api/system/mounts.html#enable-secrets-engine
    fn mount(
        &self,
        path: String,
        r#type: String,
        desc: Option<String>,
        version: Option<String>,
        config: Vec<(String, String)>,
    ) -> Result<Request<Body>, Error>;

    // https://www.vaultproject.io/api/system/mounts.html#disable-secrets-engine
    fn unmount(&self, path: String) -> Result<Request<Body>, Error>;

    // https://www.vaultproject.io/api/system/mounts.html#read-mount-configuration
    fn read_mount(&self, path: String) -> Result<Request<Body>, Error>;

    // https://www.vaultproject.io/api/system/mounts.html#tune-mount-configuration
    fn modify_mount(
        &self,
        path: String,
        lease: (usize, usize),
        audit: KeyPairs,
        display: Visibility,
        whitelist: KeyPairs,
    ) -> Result<Request<Body>, Error>;
}
