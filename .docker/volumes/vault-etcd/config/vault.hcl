storage "etcd" {
  address = "http://etcd0:2379,http://etcd1:2379,http://etcd2:2379"
  path = "/vault/"
  etcd_api = "v3"
}

listener "tcp" {
  address = "0.0.0.0:8200"
  tls_disable = 1
}
