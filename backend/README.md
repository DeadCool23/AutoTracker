# Backend документация

## Структура Backend

```ini
.
│
├── api                        # API приложения
│   ├── src                    # Исходный код API
│   └── config.cfg             # Конфиги API
├── db                         # PostresSQL 
│   ├── camera_insertions      # Сервис для вставки снимков
│   ├── data                   # Сгенерированные сущности БД
│   ├── gen_data               # Скрипт генерации данных
│   ├── loader                 # Скрипт для вставки сгенерированных данных
│   └── sql_scripts            # SQL-скрипты для поднятия БД
├── .env                       # Переменные окружения
└── docker-compose.yml         # docker-compose для поднятия БД
```
