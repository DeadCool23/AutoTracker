#!/bin/bash

set -e

start_db() {
    service redis-server start
    service postgresql start
}

prepare_postgres() {
    echo ""
    echo "=== Подготовка Postgres ==="
    echo ""

    PG_HBA_PATH="/etc/postgresql/13/main/pg_hba.conf"
    if [ -f "$PG_HBA_PATH" ]; then
        cp "$PG_HBA_PATH" "${PG_HBA_PATH}.bak"
        sed -i 's/local   all             all                                     peer/local   all             all                                     md5/g' "$PG_HBA_PATH"
        service postgresql reload
    fi

    POSTGRES_USER=$(grep '^POSTGRES_USER=' .env | cut -d '=' -f2)
    POSTGRES_PASSWORD=$(grep '^POSTGRES_PASSWORD=' .env | cut -d '=' -f2)
    POSTGRES_DB=$(grep '^POSTGRES_DB=' .env | cut -d '=' -f2)

    su - postgres -c "psql -c \"CREATE USER ${POSTGRES_USER} WITH PASSWORD '${POSTGRES_PASSWORD}';\" 2>/dev/null || true"
    su - postgres -c "psql -c \"CREATE DATABASE ${POSTGRES_DB} OWNER ${POSTGRES_USER};\""
    su - postgres -c "psql -c \"GRANT ALL PRIVILEGES ON DATABASE ${POSTGRES_DB} TO ${POSTGRES_USER};\""

    sudo -u postgres psql -c "ALTER USER ${POSTGRES_USER} WITH SUPERUSER;" 2>/dev/null || true

    cd ./db/sql_scripts/postgres

    sed -i 's/^COPY/\\copy/g' copy.sql

    for script in tables.sql constraints.sql procedure.sql functions.sql copy.sql; do
        if [ -f "$script" ]; then
            echo "Выполняется $script"
            PGPASSWORD="$POSTGRES_PASSWORD" psql -h localhost -U "$POSTGRES_USER" -d "$POSTGRES_DB" -f "$script"
        else
            echo "Предупреждение: $script не найдена"
        fi
    done
    cd ../../../
}

FLAG_FILE="/data/.db_initialized"

if [ ! -f "$FLAG_FILE" ]; then
    echo "===== ПОДГОТОВКА БАЗ ДАННЫХ (первый запуск) ====="
    
    start_db
    prepare_postgres

    touch "$FLAG_FILE"
else
    echo "===== Базы данных подготовлены ====="
    start_db
fi


cd ./api

echo ""
echo "===== СБОРКА ====="

cargo install cargo2junit

cd ./src
cargo build
cd ..

make test-report
allure open allure-report/ &

sleep 5

echo ""
echo "===== UNIT ТЕСТИРОВАНИЕ ====="

make unit-test

echo ""
echo "===== INTEGRATION ТЕСТИРОВАНИЕ ====="

make integration-test

echo ""
echo "===== E2E ТЕСТИРОВАНИЕ ====="

make e2e-test
