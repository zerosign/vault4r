use crate::proto::{error::Error, health::HealthEndpoint, lease::LeaseEndpoint};
use http::{
    method::Method,
    uri::{Authority, Scheme, Uri},
};
use hyper::{Body, Request};
use serde_json::json;

#[derive(Debug)]
pub struct Protocol {
    version: String,
    scheme: Scheme,
    authority: Authority,
}

impl HealthEndpoint for Protocol {
    #[inline]
    fn health(&self) -> Result<Request<Body>, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(format!("{}{}", self.version, Self::HEALTH_ENDPOINT).as_str())
            .build()?;

        Request::builder()
            .uri(uri)
            .body(Body::empty())
            .map_err(Error::HttpError)
    }
}

impl LeaseEndpoint for Protocol {
    #[inline]
    fn read_lease<S>(&self, id: S) -> Result<Request<Body>, Error>
    where
        S: Into<String>,
    {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(format!("{}{}", self.version, Self::READ_LEASE_ENDPOINT).as_str())
            .build()?;

        let payload = {
            let inner = json!({"lease_id" : id.into()});
            serde_json::to_string(&inner)
        }
        .map_err(Error::JsonError)?;

        Request::builder()
            .method(Method::PUT)
            .uri(uri)
            .body(Body::from(payload))
            .map_err(Error::HttpError)
    }
}
