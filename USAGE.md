# How to use

Prepare variables:

```shell
export KUBECONFIG=~/.kube/some-cluster.kubeconfig
export VAULT_ADDR=https://vault.company.com
export VAULT_TOKEN=some-token
# export VAULT_SKIP_VERIFY=true
```

Then use.

## Copy K8s secrets to the vault

```shell
# ./s2v copy --ignore-base64-errors=true <k8s-namespace> <secret-mask> <vault-dest-path>
./s2v copy --ignore-base64-errors=true demo your-app kv/demo/your-app
```

## Generate k8s secret manifests with paths to vault

Command reads secrets from cluster (variable `KUBECONFIG`) by mask (`secret-mask` argument).
Then generates yaml-manifest based on a `template.yaml` file. Secret names will be copied AS IS but value will be transformed in Vault paths.
Format `vault:<vault-dest-path>#secret-name` (Encoded with Base64).

```shell
# ./s2v gen-manifest --ignore-base64-errors=true <src-k8s-namespace> <secret-mask> <service-name> <dest-k8s-namespace> <vault-dest-path>
./s2v gen-manifest --ignore-base64-errors=true old-ns your-app your-app new-ns kv/demo/your-app
```

Notes:
- `<service-name>` represents `serviceName` variable inside `template.yaml`
- `<dest-k8s-namespace>` represents `namespace` variable inside `template.yaml`

## Append Vault secrets from source path to destination path

**PRECAUTION:** This feature is able to overwrite your secrets at destination path if you're using kv v1 engine.

```shell
# ./s2v append --ignore-base64-errors=true <vault-src-path> <vault-dest-path>
./s2v append kv/data/demo/service1-redis kv/demo/service1
```

Notes:
- Argument `vault-src-path` value it should contain `../data/..` in path
- Argument `vault-dest-path` value it should NOT contain `../data/..` in path