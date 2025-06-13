#!/bin/bash

CLICKHOUSE_HOST="localhost"
CLICKHOUSE_PORT="9000"
CLICKHOUSE_USER="nisu"
CLICKHOUSE_PASSWORD="1234"

DATA_DIR="/data"

function load_csv() {
  local table=$1
  local file=$2

  echo "Загружаем $file в таблицу $table..."

  tail -n +2 "$file" | sed 's/,True$/,1/; s/,False$/,0/; s/,True,/,1,/g; s/,False,/,0,/g' | clickhouse-client \
    --host "$CLICKHOUSE_HOST" \
    --port "$CLICKHOUSE_PORT" \
    --user "$CLICKHOUSE_USER" \
    --password "$CLICKHOUSE_PASSWORD" \
    --database "$CLICKHOUSE_DB" \
    --query="INSERT INTO $table FORMAT CSV"
}

load_csv Camera       "$DATA_DIR/cameras.csv"
load_csv CarOwner     "$DATA_DIR/owners.csv"
load_csv Car          "$DATA_DIR/cars.csv"
load_csv CarSnapshot  "$DATA_DIR/snaps.csv"
load_csv STS          "$DATA_DIR/stss.csv"
load_csv PTS          "$DATA_DIR/ptss.csv"
load_csv AppUser      "$DATA_DIR/users.csv"

echo "Загрузка завершена."
