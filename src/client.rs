// use futures::{self, future, TryFuture, TryFutureExt, TryStreamExt};
use crate::futures::{future, Future, FutureExt, Stream, TryFuture, TryFutureExt, TryStreamExt};
use hyper::{client, Body};

use crate::error::ClientError;
use crate::proto::{
    health::{HealthEndpoint, HealthInfo},
    lease::{LeaseEndpoint, LeaseStatus},
    namespace::{Namespace, NamespaceEndpoint},
    types::Protocol,
};
use crate::types::{HealthService, LeaseService, NamespaceService};
use client::connect::Connect;
use client::Client as HyperClient;

//
//
//
//
pub struct Client<C>
where
    C: Connect + 'static,
{
    inner: HyperClient<C, Body>,
    protocol: Protocol,
}

impl<C> HealthService for Client<C>
where
    C: Connect + 'static,
{
    type HealthFuture = impl TryFuture<Ok = HealthInfo, Error = ClientError>;

    #[inline]
    fn health(&self) -> Self::HealthFuture {
        let executor = self.inner.clone();

        futures::future::ready(self.protocol.health().map_err(ClientError::ProtoError))
            .and_then(move |r| executor.request(r).map_err(ClientError::HyperError))
            .and_then(|r| r.into_body().try_concat().map_err(ClientError::HyperError))
            .map(|r| match r {
                Ok(chunks) => {
                    serde_json::from_slice(&chunks.into_bytes()).map_err(ClientError::SerdeError)
                }
                Err(e) => Err(e),
            })
    }
}

impl<C> LeaseService for Client<C>
where
    C: Connect + 'static,
{
    type LeaseInfoFuture = impl TryFuture<Ok = LeaseStatus, Error = ClientError>;
    type ListLeaseFuture = impl TryFuture<Ok = Vec<String>, Error = ClientError>;
    type RevokedLeaseFuture = impl TryFuture<Ok = (), Error = ClientError>;
    type RevokedPrefixFuture = impl TryFuture<Ok = (), Error = ClientError>;

    #[inline]
    fn read_lease(&self, id: &str) -> Self::LeaseInfoFuture {
        let executor = self.inner.clone();

        future::ready(
            self.protocol
                .read_lease(id)
                .map_err(ClientError::ProtoError),
        )
        .and_then(move |r| executor.request(r).map_err(ClientError::HyperError))
        .and_then(|r| r.into_body().try_concat().map_err(ClientError::HyperError))
        .map(|r| match r {
            Ok(chunks) => {
                serde_json::from_slice(&chunks.into_bytes()).map_err(ClientError::SerdeError)
            }
            Err(e) => Err(e),
        })
    }

    #[inline]
    fn list_lease(&self, prefix: Option<&str>) -> Self::ListLeaseFuture {
        let executor = self.inner.clone();

        future::ready(
            self.protocol
                .list_lease(prefix)
                .map_err(ClientError::ProtoError),
        )
        .and_then(move |r| executor.request(r).map_err(ClientError::HyperError))
        .and_then(|r| r.into_body().try_concat().map_err(ClientError::HyperError))
        .map(|r| match r {
            Ok(chunks) => {
                serde_json::from_slice(&chunks.into_bytes()).map_err(ClientError::SerdeError)
            }
            Err(e) => Err(e),
        })
    }

    #[inline]
    fn revoke_lease(&self, id: &str) -> Self::RevokedLeaseFuture {
        let executor = self.inner.clone();

        future::ready(
            self.protocol
                .revoke_lease(id)
                .map_err(ClientError::ProtoError),
        )
        .and_then(move |r| executor.request(r).map_err(ClientError::HyperError))
        .and_then(|r| r.into_body().try_concat().map_err(ClientError::HyperError))
        .map(|r| match r {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        })
    }

    #[inline]
    fn revoke_prefix(&self, prefix: &str, forced: bool) -> Self::RevokedPrefixFuture {
        let executor = self.inner.clone();

        future::ready(
            self.protocol
                .revoke_prefix(prefix, forced)
                .map_err(ClientError::ProtoError),
        )
        .and_then(move |r| executor.request(r).map_err(ClientError::HyperError))
        .and_then(|r| r.into_body().try_concat().map_err(ClientError::HyperError))
        .map(|r| match r {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        })
    }
}

impl<C> NamespaceService for Client<C>
where
    C: Connect,
{
    type ListNamespaceFuture = impl TryFuture<Ok = Vec<String>, Error = ClientError>;
    type ShowNamespaceFuture = impl TryFuture<Ok = Namespace, Error = ClientError>;
    type CreateNamespaceFuture = impl TryFuture<Ok = (), Error = ClientError>;
    type DeleteNamespaceFuture = impl TryFuture<Ok = (), Error = ClientError>;

    #[inline]
    fn list_namespace(&self) -> Self::ListNamespaceFuture {
        let executor = self.inner.clone();

        future::ready(
            self.protocol
                .list_namespace()
                .map_err(ClientError::ProtoError),
        )
        .and_then(move |r| executor.request(r).map_err(ClientError::HyperError))
        .and_then(|r| r.into_body().try_concat().map_err(ClientError::HyperError))
        .map(|r| match r {
            Ok(chunks) => {
                serde_json::from_slice(&chunks.into_bytes()).map_err(ClientError::SerdeError)
            }
            Err(e) => Err(e),
        })
    }

    #[inline]
    fn create_namespace(&self, path: &str) -> Self::CreateNamespaceFuture {
        let executor = self.inner.clone();

        future::ready(
            self.protocol
                .create_namespace(path)
                .map_err(ClientError::ProtoError),
        )
        .and_then(move |r| executor.request(r).map_err(ClientError::HyperError))
        .and_then(|r| r.into_body().try_concat().map_err(ClientError::HyperError))
        .map(|r| match r {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        })
    }

    #[inline]
    fn delete_namespace(&self, path: &str) -> Self::DeleteNamespaceFuture {
        let executor = self.inner.clone();

        future::ready(
            self.protocol
                .delete_namespace(path)
                .map_err(ClientError::ProtoError),
        )
        .and_then(move |r| executor.request(r).map_err(ClientError::HyperError))
        .and_then(|r| r.into_body().try_concat().map_err(ClientError::HyperError))
        .map(|r| match r {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        })
    }

    #[inline]
    fn show_namespace(&self, path: &str) -> Self::ShowNamespaceFuture {
        let executor = self.inner.clone();

        future::ready(
            self.protocol
                .show_namespace(path)
                .map_err(ClientError::ProtoError),
        )
        .and_then(move |r| executor.request(r).map_err(ClientError::HyperError))
        .and_then(|r| r.into_body().try_concat().map_err(ClientError::HyperError))
        .map(|r| match r {
            Ok(chunks) => {
                serde_json::from_slice(&chunks.into_bytes()).map_err(ClientError::SerdeError)
            }
            Err(e) => Err(e),
        })
    }
}
