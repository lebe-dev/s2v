# How to use

Prepare variables:

```shell
export KUBECONFIG=~/.kube/some-cluster.kubeconfig
export VAULT_ADDR=https://vault.company.com
export VAULT_TOKEN=some-token
export VAULT_SKIP_VERIFY=true
```

Then use.

## Copy K8s secrets to the vault

```shell
./s2v copy --k8s-namespace=demo --vault-base-path=kv/demo --ignore-base64-errors=true --secret-suffixes=secret
```

### Copy and override destination vault path

This mode enabled when two options are used: `--secret-mask` and `--vault-dest-path`. Option `--vault-base-path` will be ignored.

```shell
./s2v copy --k8s-namespace=demo --vault-base-path=kv/demo \
           --ignore-base64-errors=true --secret-suffixes=secret \
           --secret-mask=manna --vault-dest-path=kv/demo/custom-dir
```

## Generate k8s secret manifests with paths to vault

Be careful with `vault-base-path` value it should contain `../data/..` after secret engine.

```shell
./s2v gen-manifests --k8s-namespace=demo --vault-base-path=kv/data/demo \
                    --secret-suffixes=secret \
                    --output-dir=manifests
```

### Override destination vault path

This mode enabled when two options are used: `--secret-mask` and `--vault-dest-path`. Option `--vault-base-path` will be ignored.

```shell
./s2v gen-manifests --k8s-namespace=demo --vault-base-path=kv/data/demo \
                    --secret-suffixes=secret --output-dir=manifests \
                    --secret-mask=manna -vault-dest-path=kv/demo/custom-dir
```

## Append Vault secrets from source path to destination path

```shell
./s2v append --vault-src-path=kv/data/demo/service1-redis --vault-dest-path=kv/demo/service1
```

Notes:
- Argument `vault-src-path` value it should contain `../data/..` in path
- Argument `vault-dest-path` value it should NOT contain `../data/..` in path

## Filter by secret name

You can specify secret name mask with `--secret-mask=[MASK]` option.

## Continue from secret

You continue 'copy' operation from specified secret name, just provide `--continue-from-secret=[SECRET-NAME]` option.