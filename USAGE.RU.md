# Как использовать

Подготовьте переменные окружения:

```shell
export KUBECONFIG=~/.kube/some-cluster.kubeconfig
export VAULT_ADDR=https://vault.company.com
export VAULT_TOKEN=some-token
# export VAULT_SKIP_VERIFY=true
```

Затем используйте команды

## Копирование секретов из K8s в HashiCorp Vault

```shell
# ./s2v copy --ignore-base64-errors=true <k8s-namespace> <secret-mask> <vault-dest-path>
./s2v copy --ignore-base64-errors=true demo your-app kv/demo
```

## Генерация манифестов для секретов с путями в HashiCorp Vault

**СТАТУС ФИЧИ:** В РАЗРАБОТКЕ

```shell
# ./s2v gen-manifests --ignore-base64-errors=true <k8s-namespace> <secret-mask> <vault-dest-path>
./s2v gen-manifests --ignore-base64-errors=true demo your-app kv/demo/your-app
```

Манифесты будут сохранены в каталог `manifests`.

## Добавление секретов из одного vault-пути к другому

**СТАТУС ФИЧИ:** В РАЗРАБОТКЕ

```shell
# ./s2v append --ignore-base64-errors=true <vault-src-path> <vault-dest-path>
./s2v append kv/data/demo/service1-redis kv/demo/service1
```

**Заметки:**
- Аргумент `vault-src-path` должен содержание значение с `../data/..`. Например, `kv/data/demo/app`
- Аргумент `vault-dest-path` НЕ должен содержание значение с `../data/..`. Например, `kv/demo/app`