#!/bin/bash

set -e

POSTGRES_ADDRESS="${POSTGRES_ADDRESS:-127.0.0.1}"
CLICKHOUSE_ADDRESS="${CLICKHOUSE_ADDRESS:-127.0.0.1}"
REDIS_ADDRESS="${REDIS_ADDRESS:-127.0.0.1}"

if [ ! -f "config.cfg" ]; then
    echo "Error: config.cfg not found!"
    exit 1
fi

cp config.cfg config.cfg.backup

sed -i.tmp \
  -e "s/postgres:\/\/nisu:1234@[0-9a-zA-Z.-]*:5432/postgres:\/\/nisu:1234@${POSTGRES_ADDRESS}:5432/g" \
  -e "s/http:\/\/nisu:1234@[0-9a-zA-Z.-]*:8123/http:\/\/nisu:1234@${CLICKHOUSE_ADDRESS}:8123/g" \
  -e "s/redis:\/\/[0-9a-zA-Z.-]*:6379/redis:\/\/${REDIS_ADDRESS}:6379/g" \
  config.cfg

rm -f config.cfg.tmp

make run