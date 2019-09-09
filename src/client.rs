use futures::{Future, IntoFuture, Stream};
use hyper::{client, Body};

use crate::error::ClientError;
use crate::proto::{
    health::{HealthEndpoint, HealthInfo},
    lease::{LeaseEndpoint, LeaseStatus},
    types::Protocol,
};
use crate::types::{HealthService, LeaseService};
use client::connect::Connect;
use client::Client as HyperClient;

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
    type HealthFuture = impl Future<Item = HealthInfo, Error = ClientError>;

    fn health(&self) -> Self::HealthFuture {
        let executor = self.inner.clone();

        self.protocol
            .health()
            .map_err(ClientError::ProtoError)
            .into_future()
            .and_then(move |r| executor.request(r).map_err(ClientError::HyperError))
            .and_then(|r| r.into_body().concat2().map_err(ClientError::HyperError))
            .and_then(|chunks| {
                serde_json::from_slice(&chunks.into_bytes()).map_err(ClientError::SerdeError)
            })
    }
}

impl<C> LeaseService for Client<C>
where
    C: Connect + 'static,
{
    type LeaseInfoFuture = impl Future<Item = LeaseStatus, Error = ClientError>;

    fn read_lease(&self, id: String) -> Self::LeaseInfoFuture {
        let executor = self.inner.clone();

        self.protocol
            .read_lease(id)
            .map_err(ClientError::ProtoError)
            .into_future()
            .and_then(move |r| executor.request(r).map_err(ClientError::HyperError))
            .and_then(|r| r.into_body().concat2().map_err(ClientError::HyperError))
            .and_then(|chunks| {
                serde_json::from_slice(&chunks.into_bytes()).map_err(ClientError::SerdeError)
            })
    }
}
