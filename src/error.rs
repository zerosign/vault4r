use crate::proto::error::Error as ProtoError;

#[derive(Debug)]
pub enum ClientError {
    ProtoError(ProtoError),
    HyperError(hyper::Error),
    SerdeError(serde_json::Error),
}
