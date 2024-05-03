# s2v

Migrate vanilla k8s secrets to HashiCorp Vault.

[На русском](README.RU.md)

[USAGE](USAGE.md) | [HOW IT WORKS](HOW-IT-WORKS.md)

## Limitations:

- Only opaque-type secrets are supported
- Only kv secret storage is supported

## Safety

Make sure you have fresh backup before you begin, but tool doesn't use any kind of delete operations in k8s and vault.

## Troubleshooting

Check `s2v.log` for details.