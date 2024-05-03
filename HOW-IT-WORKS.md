# How it works

### Copy secrets

- Tool uses `kubectl` to get secrets from namespace (`k8s-namespace` argument) with type `Opaque`
- Read secret keys and values, decode them
- Ignore secrets which contains `vault:` prefix in values
- Create secrets in vault with `vault` tool

### Append secrets from source to destination vault path

- Tool uses `vault` cli
- Read all secrets from destination vault path (ignores if path doesn't exist at the moment)
- Read all secrets from source vault path
- Merge secrets in one hashmap
- Write all secrets to the destination path

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