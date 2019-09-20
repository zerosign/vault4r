path "sys/mounts/kv-v2" {
  capabilities = [ "update" ]
}

path "sys/mounts" {
  capabilities = ["read"]
}

path "kv-v2/*" {
  capabilities = ["create", "read", "update", "delete", "list"]
}

path "sys/policies/acl/*" {
  capabilities = ["create", "read", "update", "delete", "list"]
}

path "auth/token/create" {
  capabilities = ["create", "update", "sudo"]
}
