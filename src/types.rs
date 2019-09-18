use crate::error::ClientError;
use crate::proto::health::HealthInfo;
use crate::proto::lease::LeaseStatus;
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

    fn read_lease(&self, id: String) -> Self::LeaseInfoFuture;
}
