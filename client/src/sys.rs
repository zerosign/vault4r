use http::{
    method::Method,
    uri::{Authority, Scheme, Uri},
};
use hyper::{Body, Request};
use serde_json::json;
use std::time::Duration;
use vaultr_proto::{
    error::Error,
    sys::{
        HealthEndpoint, KeyPairs, LeaseEndpoint, Mount, MountConfig, MountEndpoint,
        NamespaceEndpoint, SealEndpoint, Visibility,
    },
};

#[derive(Debug)]
pub struct SystemProtocol {
    version: String,
    scheme: Scheme,
    authority: Authority,
}

impl HealthEndpoint<Body> for SystemProtocol {
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

impl LeaseEndpoint<Body> for SystemProtocol {
    #[inline]
    fn read_lease(&self, id: &str) -> Result<Request<Body>, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(format!("{}{}", self.version, Self::READ_LEASE_ENDPOINT).as_str())
            .build()?;

        let payload = {
            let inner = json!({ "lease_id": id });
            serde_json::to_string(&inner)
        }
        .map_err(Error::JsonError)?;

        Request::builder()
            .method(Method::PUT)
            .uri(uri)
            .body(Body::from(payload))
            .map_err(Error::HttpError)
    }

    #[inline]
    fn list_lease(&self, prefix: Option<&str>) -> Result<Request<Body>, Error> {
        let query = if let Some(p) = prefix {
            format!("{}{}/{}", self.version, Self::READ_LEASE_ENDPOINT, p)
        } else {
            format!("{}{}", self.version, Self::READ_LEASE_ENDPOINT)
        };

        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(query.as_str())
            .build()?;

        Request::builder()
            .method("LIST")
            .uri(uri)
            .body(Body::empty())
            .map_err(Error::HttpError)
    }

    #[inline]
    fn renew_lease(&self, id: &str, duration: Duration) -> Result<Request<Body>, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(format!("{}{}", self.version, Self::RENEW_LEASE_ENDPOINT).as_str())
            .build()?;

        let payload = {
            let inner = json!({ "lease_id": id, "increment": duration.as_secs() });
            serde_json::to_string(&inner)
        }
        .map_err(Error::JsonError)?;

        Request::builder()
            .method(Method::PUT)
            .uri(uri)
            .body(Body::from(payload))
            .map_err(Error::HttpError)
    }

    // revoke_lease
    #[inline]
    fn revoke_lease(&self, id: &str) -> Result<Request<Body>, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(format!("{}{}", self.version, Self::REVOKE_LEASE_ENDPOINT).as_str())
            .build()?;

        let payload = {
            let inner = json!({ "lease_id": id });
            serde_json::to_string(&inner)
        }
        .map_err(Error::JsonError)?;

        Request::builder()
            .method(Method::PUT)
            .uri(uri)
            .body(Body::from(payload))
            .map_err(Error::HttpError)
    }

    #[inline]
    fn revoke_prefix(&self, prefix: &str, forced: bool) -> Result<Request<Body>, Error> {
        let endpoint = if forced {
            Self::REVOKE_LEASE_FORCE_ENDPOINT
        } else {
            Self::REVOKE_LEASE_PREFIX_ENDPOINT
        };

        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(format!("{}{}/{}", self.version, endpoint, prefix).as_str())
            .build()?;

        Request::builder()
            .method(Method::PUT)
            .uri(uri)
            .body(Body::empty())
            .map_err(Error::HttpError)
    }
}

impl NamespaceEndpoint<Body> for SystemProtocol {
    #[inline]
    fn list_namespace(&self) -> Result<Request<Body>, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(format!("{}{}", self.version, Self::NAMESPACE_ENDPOINT).as_str())
            .build()?;

        Request::builder()
            .method("LIST")
            .uri(uri)
            .body(Body::empty())
            .map_err(Error::HttpError)
    }

    #[inline]
    fn create_namespace(&self, path: &str) -> Result<Request<Body>, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(
                format!("{}{}/{}", self.version, Self::NAMESPACE_ENDPOINT, path).as_str(),
            )
            .build()?;

        Request::builder()
            .method(Method::POST)
            .uri(uri)
            .body(Body::empty())
            .map_err(Error::HttpError)
    }

    #[inline]
    fn delete_namespace(&self, path: &str) -> Result<Request<Body>, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(
                format!("{}{}/{}", self.version, Self::NAMESPACE_ENDPOINT, path).as_str(),
            )
            .build()?;

        Request::builder()
            .method(Method::DELETE)
            .uri(uri)
            .body(Body::empty())
            .map_err(Error::HttpError)
    }

    #[inline]
    fn show_namespace(&self, path: &str) -> Result<Request<Body>, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(
                format!("{}{}/{}", self.version, Self::NAMESPACE_ENDPOINT, path).as_str(),
            )
            .build()?;

        Request::builder()
            .method(Method::GET)
            .uri(uri)
            .body(Body::empty())
            .map_err(Error::HttpError)
    }
}

impl SealEndpoint<Body> for SystemProtocol {
    /// https://www.vaultproject.io/api/system/seal.html
    #[inline]
    fn seal(&self) -> Result<Request<Body>, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(format!("{}{}", self.version, Self::SEAL_ENDPOINT).as_str())
            .build()?;

        Request::builder()
            .method(Method::PUT)
            .uri(uri)
            .body(Body::empty())
            .map_err(Error::HttpError)
    }

    /// https://www.vaultproject.io/api/system/unseal.html
    #[inline]
    fn unseal(
        &self,
        key: String,
        reset: Option<bool>,
        migrate: Option<bool>,
    ) -> Result<Request<Body>, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(format!("{}{}", self.version, Self::UNSEAL_ENDPOINT).as_str())
            .build()?;

        let payload = {
            let inner = json!({ "key": key, "reset": reset.unwrap_or(false), "migrate": migrate.unwrap_or(false) });
            serde_json::to_string(&inner)
        }
        .map_err(Error::JsonError)?;

        Request::builder()
            .method(Method::PUT)
            .uri(uri)
            .body(Body::from(payload))
            .map_err(Error::HttpError)
    }

    /// https://www.vaultproject.io/api/system/seal-status.html
    #[inline]
    fn seal_info(&self) -> Result<Request<Body>, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(format!("{}{}", self.version, Self::SEAL_INFO_ENDPOINT).as_str())
            .build()?;

        Request::builder()
            .method(Method::GET)
            .uri(uri)
            .body(Body::empty())
            .map_err(Error::HttpError)
    }
}

impl MountEndpoint<Body> for SystemProtocol {
    /// https://www.vaultproject.io/api/system/mounts.html#list-mounted-secrets-engines
    #[inline]
    fn list_mounts(&self) -> Result<Request<Body>, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(format!("{}{}", self.version, Self::MOUNT_ENDPOINT).as_str())
            .build()?;

        Request::builder()
            .method(Method::GET)
            .uri(uri)
            .body(Body::empty())
            .map_err(Error::HttpError)
    }

    /// https://www.vaultproject.io/api/system/mounts.html#enable-secrets-engine
    #[inline]
    fn mount(
        &self,
        path: String,
        r#type: String,
        version: Option<String>,
        config: Vec<(String, String)>,
        desc: Option<String>,
    ) -> Result<Request<Body>, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(format!("{}{}/{}", self.version, Self::MOUNT_ENDPOINT, path).as_str())
            .build()?;

        // TODO: should convert from params to Mount
        let payload = {
            let inner = Mount::create(path, r#type, version, config, desc).unwrap();
            serde_json::to_string(&inner)
        }
        .map_err(Error::JsonError)?;

        // .map_err(Error::JsonError)?;

        Request::builder()
            .method(Method::POST)
            .uri(uri)
            .body(Body::from(payload))
            .map_err(Error::HttpError)
    }

    /// https://www.vaultproject.io/api/system/mounts.html#disable-secrets-engine
    #[inline]
    fn unmount(&self, path: String) -> Result<Request<Body>, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(format!("{}{}/{}", self.version, Self::MOUNT_ENDPOINT, path).as_str())
            .build()?;

        Request::builder()
            .method(Method::DELETE)
            .uri(uri)
            .body(Body::empty())
            .map_err(Error::HttpError)
    }

    /// https://www.vaultproject.io/api/system/mounts.html#read-mount-configuration
    #[inline]
    fn read_mount(&self, path: String) -> Result<Request<Body>, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(
                format!("{}{}/{}/tune", self.version, Self::MOUNT_ENDPOINT, path).as_str(),
            )
            .build()?;

        Request::builder()
            .method(Method::GET)
            .uri(uri)
            .body(Body::empty())
            .map_err(Error::HttpError)
    }

    /// https://www.vaultproject.io/api/system/mounts.html#tune-mount-configuration
    #[inline]
    fn modify_mount(
        &self,
        path: String,
        lease: (usize, usize),
        audit: KeyPairs,
        display: Visibility,
        whitelist: KeyPairs,
    ) -> Result<Request<Body>, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(
                format!("{}{}/{}/tune", self.version, Self::MOUNT_ENDPOINT, path).as_str(),
            )
            .build()?;

        // TODO: don't use unwrap here
        let payload = {
            let inner = MountConfig::create(path, lease, audit, display, whitelist).unwrap();
            serde_json::to_string(&inner)
        }
        .map_err(Error::JsonError)?;

        Request::builder()
            .method(Method::POST)
            .uri(uri)
            .body(Body::from(payload))
            .map_err(Error::HttpError)
    }

    /// https://www.vaultproject.io/api/system/remount.html
    #[inline]
    fn remount(&self, from: String, to: String) -> Result<Request<Body>, Error> {
        let uri = Uri::builder()
            .scheme(self.scheme.clone())
            .authority(self.authority.clone())
            .path_and_query(format!("{}{}", self.version, Self::REMOUNT_ENDPOINT).as_str())
            .build()?;

        let payload = {
            let inner = json!({
                "from" : from,
                "to" : to,
            });
            serde_json::to_string(&inner)
        }
        .map_err(Error::JsonError)?;

        Request::builder()
            .method(Method::POST)
            .uri(uri)
            .body(Body::from(payload))
            .map_err(Error::HttpError)
    }
}
