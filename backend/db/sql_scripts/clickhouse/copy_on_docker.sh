#!/bin/bash

docker exec clickhouse chmod +x /scripts/copy.sh
docker exec -it clickhouse /scripts/copy.sh

