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
# ./s2v copy [--ignore-base64-errors] [--ignore-utf8-errors] <k8s-namespace> <secret-mask> <vault-dest-path>
./s2v copy demo your-app kv/demo/your-app
```

## Генерация манифестов для секретов с путями в HashiCorp Vault

Команда читает секреты из k8s-кластера (переменная `KUBECONFIG`), которые подходят по маске (аргумент `secret-mask`).
Затем формирует yaml-манифеста на базе шаблона `template.yaml`. В качестве значений для секретов используется формат `vault:<vault-dest-path>#имя-секрета` (в кодировке base64).

```shell
# ./s2v gen-manifest [--ignore-base64-errors] [--ignore-utf8-errors] <src-k8s-namespace> <secret-mask> <service-name> <dest-k8s-namespace> <vault-dest-path>
./s2v gen-manifest old-ns your-app your-app new-ns kv/demo/your-app
```

Манифест будет сохранён в каталог `manifests` по имени сервиса (аргумент `service-name`).

- Аргумент `service-name` представлен как переменная `serviceName` в шаблоне `template.yaml`
- Аргумент `dest-k8s-namespace` представлен как переменная `namespace` в шаблоне `template.yaml`

## Добавление секретов из одного vault-пути к другому

**ПРЕДУПРЕЖДЕНИЕ:** Эта команда может перезаписать ваши секреты по указанному пути если вы используете kv v1 engine.

```shell
# ./s2v append <vault-src-path> <vault-dest-path>
./s2v append kv/data/demo/service1-redis kv/demo/service1
```

**Заметки:**
- Аргумент `vault-src-path` должен содержание значение с `../data/..`. Например, `kv/data/demo/app`
- Аргумент `vault-dest-path` НЕ должен содержание значение с `../data/..`. Например, `kv/demo/app`

## Замена базового vault-пути для секретов

Команда читает секреты из указанного файла-манифеста и меняет vault-путь. Результат печатает на экран.

```shell
# ./s2v update-vault-path [--ignore-base64-errors] [--ignore-utf8-errors] <src-manifest-file> <new-vault-path>
./s2v update-vault-path your-app.yaml kv/data/new/path/your-app
```