//!
//! https://www.vaultproject.io/api/secret/databases/index.html
//!
//!
use crate::error::Error;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleCert {
    #[serde(rename(serialize = "pem_bundle", deserialize = "pem_bundle"))]
    bundle: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BundledCert {
    ca_cert: String,
    // TODO: @zerosign (client_cert, client_key)
    client: (String, String),
    #[serde(rename(serialize = "tls_server_name", deserialize = "tls_server_name"))]
    server_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Credential {
    Basic(BasicCredential),
    SimpleCert(SimpleCert),
    BundledCert(BundledCert),
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
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Backend {
    #[serde(rename(serialize = "plugin_name", deserialize = "plugin_name"))]
    name: String,
    #[serde(rename(serialize = "allowed_roles", deserialize = "allowed_roles"))]
    roles: Vec<String>,
    #[serde(rename(
        serialize = "root_rotation_statements",
        deserialize = "root_rotation_statements"
    ))]
    rotate: Vec<String>,
    #[serde(flatten)]
    backend: Database,
}

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
    #[serde(with = "ttl")]
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

#![cfg(test)]
mod tests {

}
