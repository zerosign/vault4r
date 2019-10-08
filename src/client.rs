use crate::futures::{future, Future, FutureExt, Stream, TryFuture, TryFutureExt, TryStreamExt};
use hyper::{client, Body};

use crate::error::ClientError;
use crate::proto::{
    health::{HealthEndpoint, HealthInfo},
    lease::{LeaseEndpoint, LeaseStatus},
    namespace::{Namespace, NamespaceEndpoint},
    seal::{SealEndpoint, SealStatus},
    types::Protocol,
};
use crate::types::{HealthService, LeaseService, NamespaceService, SealService};
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

// impl <C> Client<C> where C: Connect + 'static {

//     #[inline]
//     pub(crate) fn empty_body<B>(r: http::Response<B>) -> TryFuture<Ok = B, Error = ClientError> {
//         r.into_body().try_concat().map_err(ClientError::HyperError)
//     }

//     #[inline]
//     pub(crate) fn simple_request<B>(&self, r: http::Request<B>) -> TryFuture<Ok = B, Error = ClientError> {
//         let executor = self.inner.clone();

//         future::ready(r).and_then(move |r| executor.request(r).map_err(ClientError::HyperError))
//     }
// }

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
            .map(|r| {
                r.and_then(move |chunks| {
                    serde_json::from_slice(&chunks.into_bytes()).map_err(ClientError::SerdeError)
                })
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
        .map(|r| {
            r.and_then(move |chunks| {
                serde_json::from_slice(&chunks.into_bytes()).map_err(ClientError::SerdeError)
            })
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
        .map(|r| {
            r.and_then(move |chunks| {
                serde_json::from_slice(&chunks.into_bytes()).map_err(ClientError::SerdeError)
            })
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
        .map(|r| r.map(|_| ()))
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
        .map(|r| r.map(|_| ()))
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
        .map(|r| {
            r.and_then(move |chunks| {
                serde_json::from_slice(&chunks.into_bytes()).map_err(ClientError::SerdeError)
            })
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
        .map(|r| r.map(|_| ()))
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
        .map(|r| r.map(|_| ()))
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
        .map(|r| {
            r.and_then(move |chunks| {
                serde_json::from_slice(&chunks.into_bytes()).map_err(ClientError::SerdeError)
            })
        })
    }
}

impl<C> SealService for Client<C>
where
    C: Connect,
{
    type SealFuture = impl TryFuture<Ok = (), Error = ClientError> + 'static;
    type UnsealFuture = impl TryFuture<Ok = SealStatus, Error = ClientError> + 'static;
    type SealInfoFuture = impl TryFuture<Ok = SealStatus, Error = ClientError> + 'static;

    fn seal(&self) -> Self::SealFuture {
        let executor = self.inner.clone();

        future::ready(self.protocol.seal().map_err(ClientError::ProtoError))
            .and_then(move |r| executor.request(r).map_err(ClientError::HyperError))
            .and_then(|r| r.into_body().try_concat().map_err(ClientError::HyperError))
            .map(|r| r.map(|_| ()))
    }

    fn seal_info(&self) -> Self::SealInfoFuture {
        let executor = self.inner.clone();

        future::ready(self.protocol.seal().map_err(ClientError::ProtoError))
            .and_then(move |r| executor.request(r).map_err(ClientError::HyperError))
            .and_then(|r| r.into_body().try_concat().map_err(ClientError::HyperError))
            .map(|r| {
                r.and_then(move |chunks| {
                    serde_json::from_slice(&chunks.into_bytes()).map_err(ClientError::SerdeError)
                })
            })
    }

    fn unseal(
        &self,
        key: String,
        reset: Option<bool>,
        migrate: Option<bool>,
    ) -> Self::UnsealFuture {
        let executor = self.inner.clone();

        future::ready(
            self.protocol
                .unseal(key, reset, migrate)
                .map_err(ClientError::ProtoError),
        )
        .and_then(move |r| executor.request(r).map_err(ClientError::HyperError))
        .and_then(|r| r.into_body().try_concat().map_err(ClientError::HyperError))
        .map(|r| {
            r.and_then(move |chunks| {
                serde_json::from_slice(&chunks.into_bytes()).map_err(ClientError::SerdeError)
            })
        })
    }
}
