#!/bin/bash

set -e

POSTGRES_ADDRESS="${POSTGRES_ADDRESS:-127.0.0.1}"
CLICKHOUSE_ADDRESS="${CLICKHOUSE_ADDRESS:-127.0.0.1}"
REDIS_ADDRESS="${REDIS_ADDRESS:-127.0.0.1}"

POSTGRES_PORT="${POSTGRES_PORT:-5432}"
CLICKHOUSE_PORT="${CLICKHOUSE_PORT:-8123}"
REDIS_PORT="${REDIS_PORT:-6379}"

if [ ! -f "config.cfg" ]; then
    echo "Error: config.cfg not found!"
    exit 1
fi

cp config.cfg config.cfg.backup

sed -i.tmp \
  -e "s/postgres:\/\/nisu:1234@[0-9a-zA-Z.-]*:[0-9]*\/auto_tracker/postgres:\/\/nisu:1234@${POSTGRES_ADDRESS}:${POSTGRES_PORT}\/auto_tracker/g" \
  -e "s/http:\/\/nisu:1234@[0-9a-zA-Z.-]*:[0-9]*\/auto_tracker/http:\/\/nisu:1234@${CLICKHOUSE_ADDRESS}:${CLICKHOUSE_PORT}\/auto_tracker/g" \
  -e "s/redis:\/\/[0-9a-zA-Z.-]*:[0-9]*/redis:\/\/${REDIS_ADDRESS}:${REDIS_PORT}/g" \
  config.cfg

rm -f config.cfg.tmp

echo "Config updated successfully!"
echo "   PostgreSQL: $POSTGRES_ADDRESS:$POSTGRES_PORT"
echo "   ClickHouse: $CLICKHOUSE_ADDRESS:$CLICKHOUSE_PORT"
echo "   Redis: $REDIS_ADDRESS:$REDIS_PORT"

make run