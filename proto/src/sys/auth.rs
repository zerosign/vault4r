use crate::error::Error;
use serde::Deserialize;

// TODO: @zerosign https://www.vaultproject.io/api/system/auth.html
#[derive(Debug, PartialEq, Deserialize)]
pub struct AuthSpec;

#[derive(Debug, PartialEq, Deserialize)]
pub struct AuthInfo {
    r#type: String,
    description: String,
    config: Option<AuthSpec>
}

pub trait AuthEndpoint<Payload> where Payload : Send + 'static {
    const AUTH_ENDPOINT : &'static str = "/sys/auth";

    fn list_auth(&self) -> Result<Request<Payload>, Error>;

    fn enable_auth(&self, path: String, desc: Option<String>,
                   r#type: String, spec: Option<AuthSpec>) -> Result<Request<Payload>, Error>;

    fn disable_auth(&self, path: String) -> Result<Request<Payload>, Error>;

    fn read_auth(&self, path: String) -> Result<Request<Payload>, Error>;

    fn modify_auth(&self, path: String, spec: AuthSpec) -> Result<Request<Payload>, Error>;
}

pub trait AuthService {
    type AuthError;

    type ListAuthFuture : TryFuture<Ok = Vec<AuthInfo>, Error = Self::AuthError> + 'static;
    type EnableAuthFuture: TryFuture<Ok = (), Error = Self::AuthError> + 'static;
    type DisableAuthFuture : TryFuture<Ok = (), Error = Self::AuthError> + 'static;
    type ReadAuthFuture: TryFuture<Ok = AuthSpec, Error = Self::AuthError> + 'static;
    type ModifyAuthFuture : TryFuture<Ok = (), Error = Self::AuthError> + 'static;

    fn list_auth(&self) -> Self::ListAuthFuture;

    fn enable_auth(&self, path: String, desc: Option<String>,
                   r#type: String, spec: Option<AuthSpec>) -> Self::EnableAuthFuture;

    fn disable_auth(&self, path: String) -> Self::DisableAuthFuture;

    fn read_auth(&self, path: String) -> Self::ReadAuthFuture;

    fn modify_auth(&self, path: String, spec: AuthSpec) -> Self::ModifyAuthFuture;
}
