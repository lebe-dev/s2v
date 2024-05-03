
# Как использовать

Подготовьте переменные:

```shell
export KUBECONFIG=~/.kube/some-cluster.kubeconfig
export VAULT_ADDR=https://vault.company.com
export VAULT_TOKEN=some-token
export VAULT_SKIP_VERIFY=true
```

Затем используйте.

## Копирование секретов K8s в хранилище Vault

```shell
./s2v copy --k8s-namespace=demo --vault-base-path=kv/demo --ignore-base64-errors=true --secret-suffixes=secret
```

### Копирование и перезапись пути назначения в Vault

Этот режим активируется, когда используются две опции: `--secret-mask` и `--vault-dest-path`. Опция `--vault-base-path` будет игнорироваться.

```shell
./s2v copy --k8s-namespace=demo --vault-base-path=kv/demo            --ignore-base64-errors=true --secret-suffixes=secret            --secret-mask=manna --vault-dest-path=kv/demo/custom-dir
```

## Генерация манифестов секретов k8s с путями к Vault

Будьте осторожны со значением `vault-base-path`, оно должно содержать `../data/..` после движка секретов.

```shell
./s2v gen-manifests --k8s-namespace=demo --vault-base-path=kv/data/demo                     --secret-suffixes=secret                     --output-dir=manifests
```

### Перезапись пути назначения в Vault

Этот режим активируется, когда используются две опции: `--secret-mask` и `--vault-dest-path`. Опция `--vault-base-path` будет игнорироваться.

```shell
./s2v gen-manifests --k8s-namespace=demo --vault-base-path=kv/data/demo                     --secret-suffixes=secret --output-dir=manifests                     --secret-mask=manna -vault-dest-path=kv/demo/custom-dir
```

## Добавление секретов Vault из исходного пути в путь назначения

```shell
./s2v append --vault-src-path=kv/data/demo/service1-redis --vault-dest-path=kv/demo/service1
```

Примечания:
- В значении аргумента `vault-src-path` должно содержаться `../data/..` в пути
- В значении аргумента `vault-dest-path` НЕ должно содержаться `../data/..` в пути

## Фильтрация по имени секрета

Вы можете указать маску имени секрета с помощью опции `--secret-mask=[MASK]`.

## Продолжение с указанного секрета

Вы можете продолжить операцию 'copy' с указанного имени секрета, просто предоставьте опцию `--continue-from-secret=[SECRET-NAME]`.
