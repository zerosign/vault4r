//! endpoints
//!
//!
//! `/sys/key-status` -> https://www.vaultproject.io/api/system/key-status.html
//! `/sys/rekey` -> https://www.vaultproject.io/api/system/rekey.html
//! `/sys/rekey-recovery-key` -> https://www.vaultproject.io/api/system/rekey-recovery-key.html
//! `/sys/rotate` -> https://www.vaultproject.io/api/system/rotate.html
//!
//!
use crate::error::Error;
use http::Request;
use serde::Deserialize;
use std::time;

#[derive(Debug, Deserialize, PartialEq)]
pub struct KeyStatus {
    term: usize,
    install_time: time::Instant,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct KeyRegenStatus {
    started: bool,
    nonce: String,
    t: usize,
    n: usize,
    progress: usize,
    required: usize,
    pgp_fingerprints: Vec<String>,
    backup: bool,
    verification_required: bool
}

pub trait KeyEndpoint<Payload> where Payload: Send + 'static {
    const KEY_STATUS_ENDPOINT: &'static str = "/sys/key-status";
    const REKEY_ENDPOINT: &'static str = "/sys/rekey/init";
    const REKEY_BACKUP_ENDPOINT: &'static str = "/sys/rekey/backup";
    const REKEY_UPDATE_ENDPOINT: &'static str = "/sys/rekey/update";
    const REKEY_VERIFY_ENDPOINT: &'static str = "/sys/rekey/verify";
    const ROTATE_ENDPOINT: &'static str = "/sys/rotate";

    const REKEY_REC_ENDPOINT: &'static str = "/sys/rekey-recovery-key/init";
    const REKEY_REC_BACKUP_ENDPOINT: &'static str = "/sys/rekey/recovery-key-backup";
    const REKEY_REC_UPDATE_ENDPOINT: &'static str = "/sys/rekey-recovery-key/update";
    const REKEY_REC_VERIFY_ENDPOINT: &'static str = "/sys/rekey-recovery-key/verify";

    fn key_status(&self) -> Result<Request<Payload>, Error>;

    fn regen_key_status(&self, recovery: Option<bool>) -> Result<Request<Payload>, Error>;

    fn regen_key(&self, shares: usize, threshold: usize, backup: Option<bool>, verify: Option<bool>, recovery: Option<bool>) -> Result<Request<Payload>, Error>;
    fn regen_cancel(&self, recovery: Option<bool>) -> Result<Request<Payload>, Error>;

    fn backup_key(&self, recovery: Option<bool>) -> Result<Request<Payload>, Error>;

    fn delete_backup(&self, recovery: Option<bool>) -> Result<Request<Payload>, Error>;

    fn submit_key(&self, key: String, nonce: String, recovery: Option<bool>) -> Result<Request<Payload>, Error>;

    fn regen_verify(&self, recovery: Option<bool>) -> Result<Request<Payload>, Error>;

    fn cancel_verify(&self, recovery: Option<bool>) -> Result<Request<Payload>, Error>;

    fn submit_verify(&self, key: String, nonce: String, recovery: Option<bool>) -> Result<Request<Payload>, Error>;

    fn rotate(&self) -> Result<Request<Payload>, Error>;
}

pub trait KeyService {
    type KeyError;

    type KeyStatusFuture: TryFuture<Ok = LeaseStatus, Error = Self::KeyError> + 'static;
    type RegenKeyStatusFuture: TryFuture<Ok = Vec<String>, Error = Self::KeyError> + 'static;
    type RegenKeyFuture: TryFuture<Ok = (), Error = Self::KeyError> + 'static;
    type RegenCancelFuture: TryFuture<Ok = (), Error = Self::KeyError> + 'static;
    type BackupKeyFuture: TryFuture<Ok = (), Error = Self::KeyError> + 'static;
    type DeleteBackupFuture: TryFuture<Ok = (), Error = Self::KeyError> + 'static;
    type SubmitKeyFuture: TryFuture<Ok = (), Error = Self::KeyError> + 'static;
    type RegenVerifyFuture: TryFuture<Ok = (), Error = Self::KeyError> + 'static;
    type CancelVerifyFuture: TryFuture<Ok = (), Error = Self::KeyError> + 'static;
    type SubmitVerifyFuture: TryFuture<Ok = (), Error = Self::KeyError> + 'static;
    type RotateFuture: TryFuture<Ok = (), Error = Self::KeyError + 'static>;

    fn key_status(&self, id: &str) -> Self::KeyStatusFuture;

    fn regen_key_status(&self, recovery: Option<bool>) -> Self::RegenKeyStatusFuture;

    fn regen_key(&self, shares: usize, threshold: usize, backup: Option<bool>, verify: Option<bool>, recovery: Option<bool>) -> Self::RegenKeyFuture;

    fn regen_cancel(&self, recovery: Option<bool>) -> Self::RegenCancelFuture;

    fn backup_key(&self, recovery: Option<bool>) -> Self::BackupKeyFuture;

    fn delete_backup(&self, recovery: Option<bool>) -> Self::DeleteBackupFuture;

    fn submit_key(&self, key: String, nonce: String, recovery: Option<bool>) -> Self::SubmitKeyFuture;

    fn regen_verify(&self, recovery: Option<bool>) -> Self::RegenVerifyFuture;

    fn cancel_verify(&self, recovery: Option<bool>) -> Self::CancelVerifyFuture;

    fn submit_verify(&self, key: String, nonce: String, recovery: Option<bool>) -> Self::SubmitVerifyFuture;

    fn rotate(&self) -> Self::RotateFuture;
}
