use crate::error::ClientError;
use crate::proto::health::HealthInfo;
use crate::proto::lease::LeaseStatus;
use crate::proto::namespace::Namespace;
use crate::proto::seal::SealStatus;
use futures::TryFuture;

// // Lifecycle
// //
// //
// pub trait Lifecycle {
//     fn init_status(self) -> Future<Item = bool, Error = error::Error>;

//     fn init(
//         &self,
//         keys: Option<Vec<[u8]>>,
//         root: String,
//         threshold: usize,
//     ) -> Future<Item = init::Response, Error = error::Error>;

//     fn seal(&self);
// }

//
// [doc] https://www.vaultproject.io/api/system/health.html
//
pub trait HealthService {
    //
    //
    type HealthFuture: TryFuture<Ok = HealthInfo, Error = ClientError> + 'static;

    //
    // request : GET (version)/sys/health
    //
    fn health(&self) -> Self::HealthFuture;
}

pub trait LeaseService {
    //
    //
    type LeaseInfoFuture: TryFuture<Ok = LeaseStatus, Error = ClientError> + 'static;
    type ListLeaseFuture: TryFuture<Ok = Vec<String>, Error = ClientError> + 'static;
    type RevokedLeaseFuture: TryFuture<Ok = (), Error = ClientError> + 'static;
    type RevokedPrefixFuture: TryFuture<Ok = (), Error = ClientError> + 'static;

    fn read_lease(&self, id: &str) -> Self::LeaseInfoFuture;

    fn list_lease(&self, prefix: Option<&str>) -> Self::ListLeaseFuture;

    fn revoke_lease(&self, id: &str) -> Self::RevokedLeaseFuture;

    fn revoke_prefix(&self, prefix: &str, forced: bool) -> Self::RevokedPrefixFuture;
}

pub trait NamespaceService {
    //
    //
    type ListNamespaceFuture: TryFuture<Ok = Vec<String>, Error = ClientError> + 'static;
    type ShowNamespaceFuture: TryFuture<Ok = Namespace, Error = ClientError> + 'static;
    type CreateNamespaceFuture: TryFuture<Ok = (), Error = ClientError> + 'static;
    type DeleteNamespaceFuture: TryFuture<Ok = (), Error = ClientError> + 'static;

    fn list_namespace(&self) -> Self::ListNamespaceFuture;

    fn create_namespace(&self, path: &str) -> Self::CreateNamespaceFuture;

    fn delete_namespace(&self, path: &str) -> Self::DeleteNamespaceFuture;

    fn show_namespace(&self, path: &str) -> Self::ShowNamespaceFuture;
}

pub trait SealService {
    type SealFuture: TryFuture<Ok = (), Error = ClientError> + 'static;
    type UnsealFuture: TryFuture<Ok = SealStatus, Error = ClientError> + 'static;
    type SealInfoFuture: TryFuture<Ok = SealStatus, Error = ClientError> + 'static;

    fn seal(&self) -> Self::SealFuture;

    fn unseal(&self, key: String, reset: Option<bool>, migrate: Option<bool>)
        -> Self::UnsealFuture;

    fn seal_info(&self) -> Self::SealInfoFuture;
}
