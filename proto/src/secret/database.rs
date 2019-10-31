//!
//! https://www.vaultproject.io/api/secret/databases/index.html
//!
//!
use crate::error::Error;
use serde::{de, ser, Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionDetail {
    #[serde(rename(
        serialize = "max_open_connections",
        deserialize = "max_open_connections"
    ))]
    open: usize,
    #[serde(rename(
        serialize = "max_idle_connections",
        deserialize = "max_idle_connections"
    ))]
    idle: usize,
    #[serde(rename(
        serialize = "max_lifetime_connections",
        deserialize = "max_lifetime_connections"
    ))]
    lifetime: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicCredential {
    username: String,
    password: String,
}

///
/// This could be :
/// - certificate + private key
/// - certificate + private key + ca certificate
/// - ca certificate
///
/// CAOnly | SimpleBundle | FullBundle
///
#[derive(Debug)]
pub enum BundledCert {
    CAOnly(String),
    SimpleBundle {
        certificate: String,
        private_key: String,
    },
    FullBundle {
        certificate: String,
        private_key: String,
        issuing_ca: String,
    }
}

impl <'de> Deserialize<'de> for BundledCert {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>
    {
        let mut inner = Map::deserialize(deserializer)?;

        let issuing_ca = inner.remove("issuing_ca")
            .ok_or_else(|| de::Error::missing_field("issuing_ca"))
            .and_then(Deserialize::deserialize);

        let certificate = inner.remove("certificate")
            .ok_or_else(|| de::Error::missing_field("certificate"))
            .and_then(Deserialize::deserialize);

        let private_key = inner.remove("private_key")
            .ok_or_else(|| de::Error::missing_field("private_key"))
            .and_then(Deserialize::deserialize);

        match (issuing_ca, certificate, private_key) {
            (Ok(ca), Err(_), Err(_)) => Ok(BundledCert::CAOnly(ca)),
            (Err(_), Ok(cert), Ok(key)) => Ok(BundledCert::SimpleBundle(cert, key)),
            (Ok(ca), Ok(cert), Ok(key)) => Ok(BundledCert::FullBundle(cert, key, ca)),
            _ => Err(de::Error::custom("unsupported combinations, should be either CAOnly, SimpleBundle or FullBundle")),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum Credential {
    Basic(BasicCredential),
    BundledCert(BundledCert),
}

/// ignore any options related to `insecure_tls`
/// we don't need any `insecure_tls` because it's stupid
///
/// - tls fields not exists or tls field exists with `false` ->
///   [`Credential::Basic`](Credential::Basic)
/// - tls fields exists and equal to `true` ->
///   [`Credential::BundledCert`]
///
/// [`pem_bundle`](https://www.vaultproject.io/api/secret/databases/influxdb.html#pem_bundle)
///
/// Specifies concatenated PEM blocks containing a certificate and private key; a certificate,
/// private key, and issuing CA certificate; or just a CA certificate.
///
/// This could be :
/// - certificate + private key
/// - certificate + private key + ca certificate
/// - ca certificate
///
/// [`pem_json`](https://www.vaultproject.io/api/secret/databases/influxdb.html#pem_json)
///
/// Specifies JSON containing a certificate and private key; a certificate, private key,
/// and issuing CA certificate; or just a CA certificate. For convenience format is the
/// same as the output of the issue command from the pki secrets engine.
///
impl<'de> Deserialize<'de> for Credential {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut inner = Map::deserialize(deserializer)?;

        inner.remove("tls")
            .ok_or_else(|| de::Error::missing_field("tls"))
            .and_then(Deserialize::deserialize) match {
                Ok(true) => {
                    // fetch `pem_bundle`
                    // cert -> private_key ->
                    let pem_bundle = inner.remove("pem_bundle")
                        .ok_or_else(|| de::Error::missing_field("pem_bundle"))
                        .and_then(Deserialize::deserialize)
                        .map(move|s|s.split('\n').collect::<Vec<&str>>());

                    // fetch `pem_json`
                    let pem_json = inner.remove("pem_json")
                        .ok_or_else(|| de::Error::missing_field("pem_json"))
                        // un-qouting json
                        .map(|s| format!("{}", s))
                        .and_then(Deserialize::deserialize);

                    let bundle = match (pem_bundle, pem_json) {
                        (Ok(v), Err(_)) => {
                            if v.len() == 1 {
                                // ca certificate only
                                Ok(BundledCert::CAOnly(v[0]))
                            } else if v.len() == 2 {
                                // certificate + private key
                                Ok(BundledCert::SimpleBundle(v[0], v[1]))
                            } else if v.len() == 3 {
                                Ok(BundledCert::FullBundle(v[0], v[1], v[2]))
                            } else if v.is_empty() {
                                Err(de::Error::custom("certificate bundled shouldn't be empty"))
                            } else {
                                Err(de::Error::custom(
                                    format!("invalid length of `pem_bundle`, expected 1 to 3, got {}", v.len())
                                ))
                            }
                        },
                        (Err(_), Ok(v)) => BundledCert::deserialize(v),
                        (Err(e), _) => Err(e),
                        (_, Err(e)) => Err(e),
                    };
                },
                // Ok(false) | Err(_)
                // `Credential::Basic`
                _ => {
                    BasicCredential::deserialize(Value::Object(inner)).map(Credential::Basic)
                },
            }

        Err(de::Error::custom("unimplemented!()"))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Connection {
    #[serde(rename(serialize = "connect_timeout", deserialize = "connect_timeout"))]
    timeout: usize,
    #[serde(rename(serialize = "socket_keep_alive", deserialize = "socket_keep_alive"))]
    keep_alive: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cassandra {
    hosts: Vec<String>,
    port: u32,
    #[serde(flatten)]
    credential: Credential,
    #[serde(rename(serialize = "protocol_version", deserialize = "protocol_version"))]
    version: usize,
    consistency: String,
    #[serde(flatten)]
    connection: Connection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElasticSearch {
    url: String,
    #[serde(flatten)]
    credential: Credential,
}

///
/// https://www.vaultproject.io/api/secret/databases/influxdb.html
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Influx {
    host: String,
    port: u32,
    #[serde(flatten)]
    credential: Credential,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SQL {
    #[serde(rename(serialize = "connection_url", deserialize = "connection_url"))]
    url: String,
    #[serde(flatten)]
    connection: ConnectionDetail,
    #[serde(flatten)]
    credential: BasicCredential,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MongoDB {
    #[serde(rename(serialize = "connection_url", deserialize = "connection_url"))]
    url: String,
    write_concern: String,
    #[serde(flatten)]
    credential: BasicCredential,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Database {
    SQL(SQL),
    MongoDB(MongoDB),
    ElasticSearch(ElasticSearch),
    Influx(Influx),
    Cassandra(Cassandra),
}

#[derive(Debug, Serialize)]
pub struct Backend {
    roles: Vec<String>,
    rotations: Vec<String>,
    backend: Database,
}

impl<'de> Deserialize<'de> for Backend {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut inner = Map::deserialize(deserializer)?;

        let plugin_name = inner
            .remove("plugin_name")
            .ok_or_else(|| de::Error::missing_field("plugin_name"))
            .map(Deserialize::deserialize)?
            .map_err(de::Error::custom)?;

        let roles = inner
            .remove("allowed_roles")
            .ok_or_else(|| de::Error::missing_field("allowed_roles"))
            .map(Deserialize::deserialize)?
            .map_err(de::Error::custom)?;

        let rotations = inner
            .remove("root_rotation_statements")
            .ok_or_else(|| de::Error::missing_field("root_rotation_statements"))
            .map(Deserialize::deserialize)?
            .map_err(de::Error::custom)?;

        let mut rest = Value::Object(inner);

        //
        // mysql-database-plugin
        // postgresql-database-plugin
        // oracle-database-plugin
        // mssql-database-plugin
        // mongodb-database-plugin
        // influxdb-database-plugin
        // elasticsearch-database-plugin
        // cassandra-database-plugin
        //
        match plugin_name {
            "mysql-database-plugin"
            | "postgresql-database-plugin"
            | "oracle-database-plugin"
            | "mssql-database-plugin" => Ok(Backend {
                roles: roles,
                rotations: rotations,
                backend: Database::SQL(SQL::deserialize(rest).map_err(de::Error::custom)?),
            }),
            "cassandra-database-plugin" => Ok(Backend {
                roles: roles,
                rotations: rotations,
                backend: Database::Cassandra(
                    Cassandra::deserialize(rest).map_err(de::Error::custom)?,
                ),
            }),
            "elasticsearch-database-plugin" => Ok(Backend {
                roles: roles,
                rotations: rotations,
                backend: Database::ElasticSearch(
                    ElasticSearch::deserialize(rest).map_err(de::Error::custom)?,
                ),
            }),
            "influxdb-database-plugin" => Ok(Backend {
                roles: roles,
                rotations: rotations,
                backend: Database::Influx(Influx::deserialize(rest).map_err(de::Error::custom)?),
            }),
            "mongodb-database-plugin" => Ok(Backend {
                roles: roles,
                rotations: rotations,
                backend: Database::MongoDB(MongoDB::deserialize(rest).map_err(de::Error::custom)?),
            }),
            _ => Err(de::Error::custom(format!(
                "unsupported database plugin {}",
                plugin_name
            ))),
        }
    }
}

// impl Serialize for Backend {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let backend = match &*self.backend {
//             SQL(v) => {}
//             MongoDB(v) => {}
//             ElasticSearch(v) => {}
//             Influx(v) => {}
//             Cassandra(v) => {}
//         };

//         // TODO: @zerosign
//     }
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct RoleStatement {
    #[serde(rename(serialize = "creation_statements", deserialize = "creation_statements"))]
    create: Vec<String>,
    #[serde(rename(
        serialize = "revocation_statements",
        deserialize = "revocation_statements"
    ))]
    revoke: Vec<String>,
    #[serde(rename(serialize = "rollback_statements", deserialize = "rollback_statements"))]
    rollback: Vec<String>,
    #[serde(rename(serialize = "renew_statements", deserialize = "renew_statements"))]
    renew: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Role {
    name: String,
    #[serde(rename(serialize = "db_name", deserialize = "db_name"))]
    db: String,
    // TODO: @zerosign, implement this (default_ttl, max_ttl)
    // #[serde(with = "ttl")]
    ttl: (u64, u64),
    #[serde(flatten)]
    statements: RoleStatement,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StaticRotation {
    #[serde(rename(serialize = "rotation_period", deserialize = "rotation_period"))]
    rotation: usize,
    #[serde(rename(serialize = "rotation_statements", deserialize = "rotation_statements"))]
    statement: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StaticRole {
    name: String,
    username: String,
    #[serde(rename(serialize = "db_name", deserialize = "db_name"))]
    db: String,
    #[serde(flatten)]
    rotation: StaticRotation,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    /// https://www.vaultproject.io/api/secret/databases/mysql-maria.html#sample-payload
    #[test]
    fn test_configure_connection_serde() {
        let payloads = vec![
            r#"{"plugin_name": "mysql-database-plugin", "allowed_roles": "readonly", "connection_url": "{{username}}:{{password}}@tcp(127.0.0.1:3306)/", "max_open_connections": 5, "max_connection_lifetime": "5s", "username": "root", "password": "mysql"}"#,
        ];

        let result = serde_json::from_str::<Backend>(payloads[0]);
        println!("result: {:?}", result);
        assert!(result.is_ok());
    }
}
