# API документация

TODO: Добавить ср скорость на камере у этого персонажа и скорость на камере

**Swagger path:** `{API_URL}/docs/`

## Структура API

`./src`<br>
`│` <br>
`├──` [api](./src/api/) - веб-API приложения<br>
`├──` [snap_insertions](./src/snap_insertions/) - программа для вставки изображний в БД<br>
`│` <br>
`├──` [di_container](./src/di_container/) - DI (Dependency Injection) контейнер<br>
`├──` [business_logic](./src/business_logic/) - бизнес-логика API<br>
`├──` [data_access](./src/data_access/) - уровень доступа к данным<br>
`└──` [models](./src/models/) - модели данных API<br>

## Окружение

API зависит от переменных окружения в файле [config.cnf](./config.cfg)

Пример `config.cnf` файла:

```ini
[server]
api_url = "localhost:port"                                     # Домен API

[database]
postgres_url = "postgres://user:password@localhost:port/db"    # Домен Postgres

[cache]
redis_url = "redis://localhost:port"                           # Домен Redis

[logs]
log_filename = "filename.log"                                  # Лог-файл
```

## Сборка

### Сборки

- **debug**: `make debug`
- **release**: `make release`

### Информация о сборке

```ini
=COMPILING==============================================================================
    ... # Информация Rust компиляции
=INFO===================================================================================
# release или debug зависит от сборки
Исполнямые файлы:
  • ./build/debug/api
  • ./build/debug/snap_insertions
Запуск:
  • API: [RUST_LOG=<уровень>] ./build/debug/api
  • Snap insertions: [RUST_LOG=<уровень>] ./build/debug/snap_insertions

Доступные уровни логирования:
  • RUST_LOG=error   - только критические ошибки
  • RUST_LOG=warn    - ошибки и предупреждения
  • RUST_LOG=info    - основная информация о работе [по умолчанию]
  • RUST_LOG=debug   - технические детали для разработчиков
  • RUST_LOG=trace   - максимальная детализация

Альтереативный запуск: make <snap-inserts->run BUILD=[release|debug] LOG_LEVEL=<уровень>
=========================================================================================
```

## Запуск

### Запуск API

#### с помощью Makefile

```term
make run [BUILD=<release|debug>] [LOG_LEVEL=<уровень>]
```

Используется **debug** сборка и логирование уровня **info**

#### с помощью исполняемого файла

```term
[RUST_LOG=<уровень>] ./target/<release|debug>/api
```

**RUST_LOG** при запуске можно не использовать, сборка **release|debug** на усмотрение пользователя

### Запуск snap_insertions

#### с помощью Makefile

```term
make snap-inserts-run [BUILD=<release|debug>] [LOG_LEVEL=<уровень>]
```

Используется **debug** сборка и логирование уровня **info**

#### с помощью исполняемого файла

```term
[RUST_LOG=<уровень>] ./target/<release|debug>/snap-insertions
```

**RUST_LOG** при запуске можно не использовать, сборка **release|debug** на усмотрение пользователя

## Логирование

Логи сохраняются в файл `api.log`, а также выводятся в терминал

Пример логов:

```log
[2025-04-12T13:51:08Z+03:00 INFO api] listening on 127.0.0.1:9887
[2025-04-12T13:51:20Z+03:00 INFO api::handlers::route_get_service] Received request from /car/route: RouteRequest { user_login: "example@example.com", gos_num: "А777МР77", date: "01.01.2025" }
...
[2025-04-12T13:51:20Z+03:00 WARN business_logic::services::route_service] No location data found for vehicle А777МР77 on date 01.01.2025
[2025-04-12T13:51:20Z+03:00 INFO api::handlers::route_get_service] Sended response RouteResponse {
    status: StatusResponse {
        code: 0,
        message: "OK",
    },
    route: None,
}
```
