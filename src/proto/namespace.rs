use crate::proto::error::Error;
use hyper::{Body, Request};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Namespace {
    id: String,
    path: String,
}

// https://www.vaultproject.io/api/system/namespaces.html
//
//
pub trait NamespaceEndpoint {
    const NAMESPACE_ENDPOINT: &'static str = "/sys/namespaces";

    fn list_namespace(&self) -> Result<Request<Body>, Error>;

    fn create_namespace(&self, path: &str) -> Result<Request<Body>, Error>;

    fn delete_namespace(&self, path: &str) -> Result<Request<Body>, Error>;

    fn show_namespace(&self, path: &str) -> Result<Request<Body>, Error>;
}
