#!/bin/bash

docker build -f test.Dockerfile -t testing:latest .

docker run -d --name testing testing:latest