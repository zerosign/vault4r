use crate::error::ClientError;
use crate::proto::health::HealthInfo;
use crate::proto::lease::LeaseStatus;
use crate::proto::mount::{KeyPairs, MountInfo, Visibility};
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

pub trait MountService {
    type MountFuture: TryFuture<Ok = Vec<MountInfo>, Error = ClientError> + 'static;
    type UnmountFuture: TryFuture<Ok = (), Error = ClientError> + 'static;
    type ReadMountFuture: TryFuture<Ok = MountInfo, Error = ClientError> + 'static;
    type ModifyMountFuture: TryFuture<Ok = (), Error = ClientError> + 'static;
    type RemountFuture: TryFuture<Ok = (), Error = ClientError> + 'static;

    fn mount(
        &self,
        path: String,
        r#type: String,
        desc: Option<String>,
        version: Option<String>,
        config: Vec<(String, String)>,
    ) -> Self::MountFuture;

    fn unmount(&self, path: String) -> Self::UnmountFuture;

    fn read_mount(&self, path: String) -> Self::ReadMountFuture;

    fn modify_mount(
        &self,
        path: String,
        lease: (usize, usize),
        audit: KeyPairs,
        display: Visibility,
        whitelist: KeyPairs,
    ) -> Self::ModifyMountFuture;

    fn remount(&self, from: String, to: String) -> Self::RemountFuture;
}
