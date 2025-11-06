#!/bin/bash

set -e

POSTGRES_HOST="${POSTGRES_HOST:-127.0.0.1}"
CLICKHOUSE_HOST="${CLICKHOUSE_HOST:-127.0.0.1}"
REDIS_HOST="${REDIS_HOST:-127.0.0.1}"

POSTGRES_PORT="${POSTGRES_PORT:-5432}"
CLICKHOUSE_PORT="${CLICKHOUSE_PORT:-8123}"
REDIS_PORT="${REDIS_PORT:-6379}"

IS_LOKI_LOG="${IS_LOKI_LOG:-1}"
LOKI_HOST="${LOKI_HOST:-127.0.0.1}"
LOKI_PORT="${LOKI_PORT:-3100}"
LOKI_APP_TAG="${LOKI_APP_TAG:-docker}"

if [ ! -f "config.cfg" ]; then
    echo "Error: config.cfg not found!"
    exit 1
fi

cp config.cfg config.cfg.backup

sed -i.tmp \
  -e "s/postgres:\/\/nisu:1234@[0-9a-zA-Z.-]*:[0-9]*\/auto_tracker/postgres:\/\/nisu:1234@${POSTGRES_HOST}:${POSTGRES_PORT}\/auto_tracker/g" \
  -e "s/http:\/\/nisu:1234@[0-9a-zA-Z.-]*:[0-9]*\/auto_tracker/http:\/\/nisu:1234@${CLICKHOUSE_HOST}:${CLICKHOUSE_PORT}\/auto_tracker/g" \
  -e "s/redis:\/\/[0-9a-zA-Z.-]*:[0-9]*/redis:\/\/${REDIS_HOST}:${REDIS_PORT}/g" \
  -e "s/^is_loki_log *= *\"[0-9]*\"/is_loki_log = \"${IS_LOKI_LOG}\"/g" \
  -e "s|^loki_url *= *\"[^\"]*\"|loki_url = \"http://${LOKI_HOST}:${LOKI_PORT}\"|g" \
  -e "s/^loki_app_tag *= *\"[^\"]*\"/loki_app_tag = \"${LOKI_APP_TAG}\"/g" \
  config.cfg

rm -f config.cfg.tmp

echo "Config updated successfully!"
cat config.cfg

make fast-run