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
./s2v copy --ignore-base64-errors=true demo your-app kv/demo
```

## Generate k8s secret manifests with paths to vault

**FEATURE STATUS:** UNDER DEVELOPMENT

Create secret manifests with vault paths as values. Output directory `manifests`

```shell
# ./s2v gen-manifests --ignore-base64-errors=true <k8s-namespace> <secret-mask> <vault-dest-path>
./s2v gen-manifests --ignore-base64-errors=true demo your-app kv/demo/your-app
```

## Append Vault secrets from source path to destination path

**PRECAUTION:** This feature is able to overwrite your secrets at destination path.

```shell
# ./s2v append --ignore-base64-errors=true <vault-src-path> <vault-dest-path>
./s2v append kv/data/demo/service1-redis kv/demo/service1
```

Notes:
- Argument `vault-src-path` value it should contain `../data/..` in path
- Argument `vault-dest-path` value it should NOT contain `../data/..` in path