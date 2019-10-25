# vaultr

client library for vault using only `hyper`. Most of the types are carefully handcrafted rather than generated
by their `swagger`spec API.

Currently, this library focuses into implementing http api for most of the `/sys/*` backend except `wrapping`, `raw` & `tools`.
This library also support (most) listed secret engines in vaults.

This library heavily uses `type_alias_impl_trait` features in rust. Most of the endpoints are being implemented in
traits.

This library prone to huge changes for stabilizations, please be wary.

Notes: I sometimes confuse whether I need to expose the API as per individual API service or full facade API per backend like this.
Since some API has really huge surface definitions (`sys/*`).

# Development

The library are being splitted into 2 library, `proto` & `client`. Most enterprise only API are not being supported.

Implemented system backend endpoints :

- [x] Health endpoints (`/sys/health`)
- [x] Lease endpoints (`/sys/leases`)
- [x] Namespace endpoints (`/sys/namespaces`)
- [x] Seal/unseal endpoints (`/sys/seal`, `/sys/unseal`, `/sys/seal-status`)
- [x] Mount endpoints (`/sys/mounts/*`)
- [x] Init endpoints (`/sys/init/*`)
- [x] Capability endpoints (`/sys/capabilities`, `/sys/capabilities-self`, `/sys/capabilities-accessor`)
- [-] Control Group endpoints (`/sys/config/control-group`, `/sys/control-group`)
- [ ] CORS endpoints
- [+] Key endpoints (`/sys/key-status`, `/sys/rekey`, `/sys/rekey-recovery-key`, `/sys/rotate`)
- [ ] Replication/Cluster endpoints (`/sys/replication/*`, `/sys/leader`, `/sys/step-down`, `/sys/storage`)
- [+] Policy endpoints (`/sys/policy`, `/sys/policies`)
- [ ] Plugins endpoints (`/sys/plugins/reload`, `/sys/plugins/catalog`)
- [ ] Audit endpoints (`/sys/audit`, `/sys/audit-hash`, `/sys/config/auditing`)

Implemented secret engines :

- [ ] Cubbyhole
- [ ] Databases
  - [ ] Cassandra
  - [ ] Elasticsearch
  - [ ] MongoDB
  - [ ] MySQL/MariaDB
  - [ ] PostgreSQL
- [ ] Google Cloud
- [ ] Google Cloud KMS
- [ ] AWS
- [ ] Alicloud
- [ ] Identity
- [ ] Kev Value (I & II)
- [ ] PKI
- [ ] SSH
- [ ] TOTP
- [ ] Transit
- [ ] RabbitMQ

# TODO

- [ ] Auto test with most recent tagged branch of `vault`
- [ ] Add some criteria testing for most of the newest version of `vault`
