use crate::error::ClientError;
use crate::proto::health::HealthInfo;
use crate::proto::lease::LeaseStatus;
use futures::Future;

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
    type HealthFuture: Future<Item = HealthInfo, Error = ClientError>;

    //
    // request : GET (version)/sys/health
    //
    fn health(&self) -> Self::HealthFuture;
}

pub trait LeaseService {
    type LeaseInfoFuture: Future<Item = LeaseStatus, Error = ClientError>;

    fn read_lease(&self, id: String) -> Self::LeaseInfoFuture;
}
