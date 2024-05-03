# s2v

Migrate vanilla k8s secrets to HashiCorp Vault.

[На русском](README.RU.md)

[USAGE](USAGE.md) | [HOW IT WORKS](HOW-IT-WORKS.md)

## Limitations:

- Only opaque-type secrets are supported
- Only kv secret storage is supported

## Data safety

The tool doesn't use direct remove command in kubernetes or vault, but `append` command 
is able to overwrite secrets at destination vault path. Use it mindfully.

## Troubleshooting

Check `s2v.log` for details.