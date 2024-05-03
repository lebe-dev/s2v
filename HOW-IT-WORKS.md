# How it works

### Copy secrets

- Tool uses `kubectl` to get secrets from namespace (`--k8s-namespace` option) with type `Opaque`
- Read secret keys and values, decode them
- Ignore secrets which contains `vault:` prefix in values
- Remove from secret name suffixes (`--secret-suffixes` option) to get service name. I.e. `postgres-secret` -> `postgres`.
- Create secrets in vault with `vault` cli tool. Vault secret path in format '[VAULT-BASE-PATH]/[SERVICE-NAME]' (depends on `--vault-base-path` option)

### Append secrets from source to destination vault path

- Tool uses `vault` cli
- Read all secrets from destination vault path
- Read all secrets from source vault path
- Merge secrets in one hashmap
- Write all secrets to destination path

### Secrets decoding and creating in Vault

For example, we have secret resource with name `app-secret` in namespace `demo`.

Secret contains values:

- `DATABASE_URL`
- `DATABASE_USER`
- `DATABASE_PASSWORD`

`s2v` will create secrets:

```
kv/demo/app#DATABASE_URL
kv/demo/app#DATABASE_USER
kv/demo/app#DATABASE_PASSWORD
```