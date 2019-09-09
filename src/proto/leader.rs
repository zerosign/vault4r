//
//
//
#[derive(Debug, Deserialize)]
pub struct LeaderInfo {
    leader: String,
    cluster: String,
}

//
// {
//   "ha_enabled": true,
//   "is_self": false,
//   "leader_address": "https://127.0.0.1:8200/",
//   "leader_cluster_address": "https://127.0.0.1:8201/",
//   "performance_standby": false,
//   "performance_standby_last_remote_wal": 0
// }
//
#[derive(Debug, Deserialize)]
pub struct Response {
    ha_enabled: bool,
    is_self: bool,
    leader: LeaderInfo,
    standby: false,
    standby_last_remote_wal: usize,
}
