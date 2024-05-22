#!/bin/bash

# Copy secrets from k8s cluster to the Vault instance,
# generates the secret manifest with vault paths

export VAULT_ADDR=https://CHANGE-ME
export VAULT_TOKEN=CHANGE-ME
export KUBECONFIG=~/.kube/prod.kubeconfig

if [ $# -ne 3 ]; then
  echo "Usage: $0 <namespace> <service_name> <secret_mask>"
  exit 1
fi

namespace=$1
service_name=$2
secret_mask=$3

# ./s2v copy [--ignore-base64-errors] [--ignore-utf8-errors] <k8s-namespace> <secret-mask> <vault-dest-path>
./s2v --log-level=trace copy --ignore-base64-errors --ignore-utf8-errors ${namespace} ${secret_mask} kv/${namespace}/${service_name}

echo "creating vault policy.."

vault policy write ${service_name} - <<EOF
path "kv/data/${namespace}/${service_name}" {
  capabilities = ["read"]
}
EOF

vault write auth/kubernetes/role/${service_name} policies=${service_name} bound_service_account_names=${service_name},vault,vault-secrets-webhook bound_service_account_namespaces=${namespace},vault ttl=24h

echo "creating manifest file.."

# ./s2v gen-manifest [--ignore-base64-errors] [--ignore-utf8-errors] <src-k8s-namespace> <secret-mask> <service-name> <dest-k8s-namespace> <vault-dest-path>
./s2v gen-manifest --ignore-base64-errors --ignore-utf8-errors ${namespace} ${secret_mask} ${service_name} ${namespace} kv/${namespace}/${service_name}

echo "---"
echo "done"