#!/bin/bash

#
# Rename service name everywhere
#
# - Append vault secrets from source path to the destination path
# - Create vault policy

if [ $# -ne 3 ]; then
  echo "Usage: $0 <namespace> <src-service> <dest-service>"
  exit 1
fi

namespace=$1
src_service=$2
dest_service=$3

./s2v append kv/data/${namespace}/${src_service} kv/${namespace}/${dest_service}
./s2v append kv/data/${namespace}/${src_service}-redis kv/${namespace}/${dest_service}

vault policy write ${dest_service} - <<EOF
path "kv/data/${namespace}/${dest_service}" {
  capabilities = ["read"]
}
EOF

vault write auth/kubernetes/role/${dest_service} policies=${dest_service} bound_service_account_names=${dest_service},vault,vault-secrets-webhook bound_service_account_namespaces=${namespace},vault ttl=24h

echo "---"
echo "done"