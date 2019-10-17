use crate::{error::Error, sys::SystemProtocol};
use client::{connect::Connect, Client as HyperClient};
use futures::{future, Future, FutureExt, Stream, TryFuture, TryFutureExt, TryStreamExt};
use hyper::{client, Body};
use vaultr_proto::{
    error::Error as ProtoError,
    sys::{
        HealthEndpoint, HealthInfo, HealthService, KeyPairs, LeaseEndpoint, LeaseService,
        LeaseStatus, MountEndpoint, MountInfo, MountService, Namespace, NamespaceEndpoint,
        NamespaceService, SealEndpoint, SealService, SealStatus, Visibility,
    },
};

pub struct Client<C>
where
    C: Connect + 'static,
{
    inner: HyperClient<C, Body>,
    protocol: SystemProtocol,
}

impl<C> HealthService for Client<C>
where
    C: Connect + 'static,
{
    type HealthError = Error;
    type HealthFuture = impl TryFuture<Ok = HealthInfo, Error = Self::HealthError>;

    #[inline]
    fn health(&self) -> Self::HealthFuture {
        let executor = self.inner.clone();

        futures::future::ready(
            self.protocol
                .health()
                .map_err(Self::HealthError::ProtoError),
        )
        .and_then(move |r| executor.request(r).map_err(Self::HealthError::HyperError))
        .and_then(|r| {
            r.into_body()
                .try_concat()
                .map_err(Self::HealthError::HyperError)
        })
        .map(|r| {
            r.and_then(move |chunks| {
                serde_json::from_slice(&chunks.into_bytes()).map_err(Self::HealthError::SerdeError)
            })
        })
    }
}

impl<C> LeaseService for Client<C>
where
    C: Connect + 'static,
{
    type LeaseError = Error;

    type LeaseInfoFuture = impl TryFuture<Ok = LeaseStatus, Error = Self::LeaseError>;
    type ListLeaseFuture = impl TryFuture<Ok = Vec<String>, Error = Self::LeaseError>;
    type RevokedLeaseFuture = impl TryFuture<Ok = (), Error = Self::LeaseError>;
    type RevokedPrefixFuture = impl TryFuture<Ok = (), Error = Self::LeaseError>;

    #[inline]
    fn read_lease(&self, id: &str) -> Self::LeaseInfoFuture {
        let executor = self.inner.clone();

        future::ready(
            self.protocol
                .read_lease(id)
                .map_err(Self::LeaseError::ProtoError),
        )
        .and_then(move |r| executor.request(r).map_err(Self::LeaseError::HyperError))
        .and_then(|r| {
            r.into_body()
                .try_concat()
                .map_err(Self::LeaseError::HyperError)
        })
        .map(|r| {
            r.and_then(move |chunks| {
                serde_json::from_slice(&chunks.into_bytes()).map_err(Self::LeaseError::SerdeError)
            })
        })
    }

    #[inline]
    fn list_lease(&self, prefix: Option<&str>) -> Self::ListLeaseFuture {
        let executor = self.inner.clone();

        future::ready(
            self.protocol
                .list_lease(prefix)
                .map_err(Self::LeaseError::ProtoError),
        )
        .and_then(move |r| executor.request(r).map_err(Self::LeaseError::HyperError))
        .and_then(|r| {
            r.into_body()
                .try_concat()
                .map_err(Self::LeaseError::HyperError)
        })
        .map(|r| {
            r.and_then(move |chunks| {
                serde_json::from_slice(&chunks.into_bytes()).map_err(Self::LeaseError::SerdeError)
            })
        })
    }

    #[inline]
    fn revoke_lease(&self, id: &str) -> Self::RevokedLeaseFuture {
        let executor = self.inner.clone();

        future::ready(
            self.protocol
                .revoke_lease(id)
                .map_err(Self::LeaseError::ProtoError),
        )
        .and_then(move |r| executor.request(r).map_err(Self::LeaseError::HyperError))
        .and_then(|r| {
            r.into_body()
                .try_concat()
                .map_err(Self::LeaseError::HyperError)
        })
        .map(|r| r.map(|_| ()))
    }

    #[inline]
    fn revoke_prefix(&self, prefix: &str, forced: bool) -> Self::RevokedPrefixFuture {
        let executor = self.inner.clone();

        future::ready(
            self.protocol
                .revoke_prefix(prefix, forced)
                .map_err(Self::LeaseError::ProtoError),
        )
        .and_then(move |r| executor.request(r).map_err(Self::LeaseError::HyperError))
        .and_then(|r| {
            r.into_body()
                .try_concat()
                .map_err(Self::LeaseError::HyperError)
        })
        .map(|r| r.map(|_| ()))
    }
}
