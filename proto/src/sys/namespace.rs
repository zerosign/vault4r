use crate::error::Error;
use futures::TryFuture;
use http::Request;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Namespace {
    id: String,
    path: String,
}

// https://www.vaultproject.io/api/system/namespaces.html
//
pub trait NamespaceEndpoint<Payload>
where
    Payload: Send + 'static,
{
    const NAMESPACE_ENDPOINT: &'static str = "/sys/namespaces";

    fn list_namespace(&self) -> Result<Request<Payload>, Error>;
    fn create_namespace(&self, path: &str) -> Result<Request<Payload>, Error>;
    fn delete_namespace(&self, path: &str) -> Result<Request<Payload>, Error>;
    fn show_namespace(&self, path: &str) -> Result<Request<Payload>, Error>;
}

pub trait NamespaceService {
    type NamespaceError;

    type ListNamespaceFuture: TryFuture<Ok = Vec<String>, Error = Self::NamespaceError> + 'static;
    type ShowNamespaceFuture: TryFuture<Ok = Namespace, Error = Self::NamespaceError> + 'static;
    type CreateNamespaceFuture: TryFuture<Ok = (), Error = Self::NamespaceError> + 'static;
    type DeleteNamespaceFuture: TryFuture<Ok = (), Error = Self::NamespaceError> + 'static;

    fn list_namespace(&self) -> Self::ListNamespaceFuture;

    fn create_namespace(&self, path: &str) -> Self::CreateNamespaceFuture;

    fn delete_namespace(&self, path: &str) -> Self::DeleteNamespaceFuture;

    fn show_namespace(&self, path: &str) -> Self::ShowNamespaceFuture;
}
