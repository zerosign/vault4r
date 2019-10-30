use crate::error::Error;
use futures::TryFuture;
use http::Request;
use serde::{self, de, ser, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ClusterInfo {
    #[serde(rename(deserialize = "cluster_id", serialize = "cluster_id"))]
    id: String,
    #[serde(rename(deserialize = "cluster_name", serialize = "cluster_name"))]
    name: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SealInfo {
    r#type: Option<String>,
    #[serde(rename(deserialize = "t", serialize = "t"))]
    threshold: usize,
    #[serde(rename(deserialize = "n", serialize = "n"))]
    shares: usize,
    progress: usize,
    nonce: Option<String>,
    version: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Unsealed {
    #[serde(flatten)]
    info: SealInfo,
    #[serde(flatten)]
    cluster: ClusterInfo,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Sealed {
    #[serde(flatten)]
    info: SealInfo,
}

#[derive(Debug, PartialEq)]
pub enum SealStatus {
    Sealed(Sealed),
    Unsealed(Unsealed),
}

///
/// This is workaround for deserialize since serde_derive didn't
/// support literal for tagged value deserialization and serialization.
///
/// NOTE: This will be superseeded by https://github.com/serde-rs/serde/pull/1392
///
impl<'de> Deserialize<'de> for SealStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut inner = Map::deserialize(deserializer)?;

        let status = inner
            .remove("sealed")
            .ok_or_else(|| de::Error::missing_field("sealed"))
            .map(Deserialize::deserialize)?
            .map_err(de::Error::custom)?;

        let rest = Value::Object(inner);

        if status {
            Sealed::deserialize(rest)
                .map(SealStatus::Sealed)
                .map_err(de::Error::custom)
        } else {
            Unsealed::deserialize(rest)
                .map(SealStatus::Unsealed)
                .map_err(de::Error::custom)
        }
    }
}

impl Serialize for SealStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match &*self {
            Self::Sealed(v) => (true, serde_json::to_value(v).map_err(ser::Error::custom)?),
            Self::Unsealed(v) => (false, serde_json::to_value(v).map_err(ser::Error::custom)?),
        };

        let value = match value {
            (flag, Value::Object(mut h)) => {
                h.insert(String::from("sealed"), Value::Bool(flag));
                Ok(Value::Object(h))
            }
            _ => Err(ser::Error::custom("not a map")),
        }?;

        value.serialize(serializer)
    }
}

pub trait SealEndpoint<Payload>
where
    Payload: Send + 'static,
{
    const SEAL_ENDPOINT: &'static str = "/sys/seal";
    const UNSEAL_ENDPOINT: &'static str = "/sys/unseal";
    const SEAL_INFO_ENDPOINT: &'static str = "/sys/seal-status";

    /// https://www.vaultproject.io/api/system/seal.html
    ///
    ///
    fn seal(&self) -> Result<Request<Payload>, Error>;

    /// https://www.vaultproject.io/api/system/unseal.html
    ///
    fn unseal(
        &self,
        key: String,
        reset: Option<bool>,
        migrate: Option<bool>,
    ) -> Result<Request<Payload>, Error>;

    /// https://www.vaultproject.io/api/system/seal-status.html
    ///
    fn seal_info(&self) -> Result<Request<Payload>, Error>;
}

pub trait SealService {
    type SealError;

    type SealFuture: TryFuture<Ok = (), Error = Self::SealError> + 'static;
    type UnsealFuture: TryFuture<Ok = SealStatus, Error = Self::SealError> + 'static;
    type SealInfoFuture: TryFuture<Ok = SealStatus, Error = Self::SealError> + 'static;

    fn seal(&self) -> Self::SealFuture;

    fn unseal(&self, key: String, reset: Option<bool>, migrate: Option<bool>)
        -> Self::UnsealFuture;

    fn seal_info(&self) -> Self::SealInfoFuture;
}

#[cfg(test)]
mod test {
    use super::{ClusterInfo, SealInfo, SealStatus, Sealed, Unsealed};
    use serde_json;

    #[test]
    fn serde_status_test_serde() {
        let raw = [
            r#"{"sealed": true,"t": 3, "n": 5, "progress": 2, "version": "0.6.2"}"#,
            r#"{"sealed": false, "t": 3, "n": 5, "progress": 0, "version": "0.6.2", "cluster_name": "vault-cluster-d6ec3c7f", "cluster_id": "3e8b3fec-3749-e056-ba41-b62a63b997e8"}"#,
        ];

        let exp: Vec<_> = vec![
            SealStatus::Sealed(Sealed {
                info: SealInfo {
                    r#type: None,
                    threshold: 3,
                    shares: 5,
                    progress: 2,
                    nonce: None,
                    version: String::from("0.6.2"),
                },
            }),
            SealStatus::Unsealed(Unsealed {
                info: SealInfo {
                    r#type: None,
                    threshold: 3,
                    shares: 5,
                    progress: 0,
                    nonce: None,
                    version: String::from("0.6.2"),
                },
                cluster: ClusterInfo {
                    id: String::from("3e8b3fec-3749-e056-ba41-b62a63b997e8"),
                    name: String::from("vault-cluster-d6ec3c7f"),
                },
            }),
        ];

        for (idx, response) in raw.iter().enumerate() {
            let result = serde_json::from_str::<SealStatus>(response);
            assert!(result.is_ok());
            let inner = result.unwrap();
            assert_eq!(inner, exp[idx]);

            let serialized = serde_json::to_string(&inner);
            assert!(serialized.is_ok());
            let result2 = serde_json::from_str::<SealStatus>(&serialized.unwrap());
            assert!(result2.is_ok());
            assert_eq!(result2.unwrap(), inner);
        }
    }
}
