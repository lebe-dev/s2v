# s2v

Migrate vanilla k8s secrets to HashiCorp Vault.

[На русском](README.RU.md)

[USAGE](USAGE.md) | [HOW IT WORKS](HOW-IT-WORKS.md)

## Features:

- Copy secrets from Kubernetes to HashiCorp Vault
- Generate secret manifest with vault paths as values
- Append secrets from source vault path to another
- Update vault paths for given secret manifest file, print results to stdout

## Limitations:

- Only opaque-type secrets are supported
- Only kv secret storage is supported

## Data safety

The tool doesn't use direct remove command in kubernetes or vault, but `append` command 
is able to overwrite secrets at destination vault path if you don't use [kv v2 engine](https://developer.hashicorp.com/vault/tutorials/secrets-management/versioned-kv?variants=vault-deploy%3Aselfhosted). Use it mindfully.

## Security

Logging level `trace` contains sensitive output so remove `s2v.log` file after all.

## Troubleshooting

Check `s2v.log` for details.